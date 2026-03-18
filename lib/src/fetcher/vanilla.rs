use serde::Deserialize;
use crate::fetcher::{ServerFetcher, FetcherContext};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: Minecraft Vanillaの公式サーバーを取得するFetcher
 */
pub struct VanillaFetcher {
    ctx: FetcherContext,
}

#[derive(Deserialize)]
struct Manifest {
    versions: Vec<ManifestVersion>,
}

#[derive(Deserialize)]
struct ManifestVersion {
    id: String,
    url: String,
    #[serde(rename = "type")]
    version_type: String,
}

#[derive(Deserialize)]
struct VersionPackage {
    downloads: Downloads,
}

#[derive(Deserialize)]
struct Downloads {
    server: DownloadInfo,
}

#[derive(Deserialize)]
struct DownloadInfo {
    url: String,
}

impl VanillaFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl ServerFetcher for VanillaFetcher {
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        let resp: Manifest = self.ctx.client.get("https://launchermeta.mojang.com/mc/game/version_manifest.json").send().await?.json().await?;
        Ok(resp.versions.into_iter().map(|v| v.id).collect())
    }

    async fn get_latest_version(&self) -> Result<String, MsbError> {
        let resp: Manifest = self.ctx.client.get("https://launchermeta.mojang.com/mc/game/version_manifest.json").send().await?.json().await?;
        resp.versions.iter()
            .find(|v| v.version_type == "release")
            .map(|v| v.id.clone())
            .ok_or_else(|| MsbError::ParseError("No release versions found".to_string()))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        let manifest: Manifest = self.ctx.client.get("https://launchermeta.mojang.com/mc/game/version_manifest.json").send().await?.json().await?;
        let v_entry = manifest.versions.iter()
            .find(|v| v.id == version)
            .ok_or(MsbError::VersionNotFound(version.to_string()))?;
        
        let pkg: VersionPackage = self.ctx.client.get(&v_entry.url).send().await?.json().await?;
        Ok(pkg.downloads.server.url)
    }

    async fn get_specific_build_url(&self, version: &str, _build: u32) -> Result<String, MsbError> {
        self.get_download_url(version).await
    }

    /**
     * 説明: ストリーミング方式でJARをダウンロードする (デコードエラー回避)
     */
    async fn download_jar(&self, url: &str, output_path: &Path) -> Result<(), MsbError> {
        let response = self.ctx.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(MsbError::HttpError(format!("ダウンロード失敗: {}", response.status())));
        }

        let mut file = File::create(output_path).await?;
        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let data = chunk.map_err(|e| MsbError::NetworkError(e.to_string()))?;
            file.write_all(&data).await?;
        }
        file.flush().await?;
        Ok(())
    }
}
