use crate::fetcher::FetcherContext;
use crate::error::MsbError;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;
use futures_util::StreamExt;

/**
 * 説明: Adoptium APIを使用してJDKをダウンロード・管理するFetcher
 */
pub struct JdkFetcher {
    ctx: FetcherContext,
}

impl JdkFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }

    /**
     * 説明: Minecraftバージョンやエンジンの種類に基づいて、最適なJavaバージョンを決定する
     */
    pub fn get_recommended_java_version(mc_version: &str, server_type: &str) -> u32 {
        let server_type = server_type.to_lowercase();
        
        // 💡 Arclight は常に Java 21 を要求する傾向がある
        if server_type.contains("arclight") {
            return 21;
        }

        // 💡 最新バージョンや特定のエンジンは Java 21 必須
        if mc_version.contains("1.20.5") || mc_version.contains("1.21") || server_type.contains("folia") {
            return 21;
        }

        // 💡 1.18〜1.20 は Java 17
        if mc_version.contains("1.18") || mc_version.contains("1.19") || mc_version.contains("1.20") {
            return 17;
        }

        // 💡 1.17 は Java 16
        if mc_version.contains("1.17") {
            return 17; // 16はEOLに近いので17で代用
        }

        // 💡 それ以前でも、Velocity等のプロキシを動かすなら最低 17 を推奨
        17 
    }

    /**
     * 説明: JDKをダウンロードして展開する。失敗時はファイルを削除してクリーンな状態を保つ。
     */
    pub async fn download_and_extract_jdk(&self, java_version: u32, target_dir: &Path) -> Result<PathBuf, MsbError> {
        let os = if cfg!(windows) { "windows" } else { "linux" };
        let arch = "x64"; 
        let url = format!(
            "https://api.adoptium.net/v3/binary/latest/{java_version}/ga/{os}/{arch}/jdk/hotspot/normal/eclipse?project=jdk"
        );

        println!("☕ JDK {java_version} ({os}) をストリーミング取得中...");
        
        let response = self.ctx.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(MsbError::HttpError(format!("JDK取得失敗: {}", response.status())));
        }

        let temp_file = target_dir.join(format!("jdk_{java_version}.zip"));
        
        let mut f = File::create(&temp_file).await?;
        let mut stream = response.bytes_stream();
        
        let download_res = async {
            while let Some(chunk) = stream.next().await {
                let data = chunk.map_err(|e| MsbError::NetworkError(e.to_string()))?;
                f.write_all(&data).await?;
            }
            f.flush().await?;
            Ok::<(), MsbError>(())
        }.await;

        if let Err(e) = download_res {
            let _ = fs::remove_file(&temp_file).await;
            return Err(e);
        }

        println!("📦 JDK を展開中...");
        let extract_path = target_dir.join(format!("jdk-{java_version}"));
        if !extract_path.exists() { fs::create_dir_all(&extract_path).await?; }

        let temp_file_c = temp_file.clone();
        let extract_path_c = extract_path.clone();

        tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(&temp_file_c)?;
            let mut archive = ZipArchive::new(file).map_err(|_| MsbError::ParseError("Invalid ZIP format".to_string()))?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|_| MsbError::ParseError("Zip error".to_string()))?;
                let enclosed_name = file.enclosed_name().ok_or(MsbError::ParseError("Unsafe zip path".to_string()))?.to_path_buf();
                let outpath = extract_path_c.join(enclosed_name);
                
                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() { std::fs::create_dir_all(p)?; }
                    }
                    let mut outfile = std::fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
            Ok::<(), MsbError>(())
        }).await.map_err(|_| MsbError::Unknown("Blocking task failed".to_string()))??;

        let _ = fs::remove_file(&temp_file).await;

        let bin_dir = if cfg!(windows) {
            find_java_executable(&extract_path).ok_or(MsbError::ParseError("Java executable not found".to_string()))?
        } else {
            extract_path.join("bin").join("java")
        };

        Ok(bin_dir)
    }
}

fn find_java_executable(dir: &Path) -> Option<PathBuf> {
    for entry in std::fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(p) = find_java_executable(&path) { return Some(p); }
        } else {
            let file_name = path.file_name()?.to_string_lossy();
            if file_name == "java.exe" || (cfg!(unix) && file_name == "java" && path.parent()?.ends_with("bin")) {
                return Some(path);
            }
        }
    }
    None
}
