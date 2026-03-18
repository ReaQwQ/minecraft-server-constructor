use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use crate::error::MsbError;
use crate::fetcher::jar::JarExtractor;

/**
 * 説明: バックエンドサーバーの設定を最適化（シークレットの反映、JARからのプロパティ取得等）する
 * @param dir サーバーのルートディレクトリ
 * @param jar_path サーバーJARのパス
 * @param secret プロキシ連携用の共有秘密鍵
 * @requires tokio::fs, JarExtractor
 * @return 成功時はOk(())
 */
pub async fn optimize_backend_config(dir: &Path, jar_path: &Path, secret: &str) -> Result<(), MsbError> {
    // 1. server.properties の基本設定
    let mut props = String::from("online-mode=false\nenable-status=true\nnetwork-compression-threshold=256\n");
    
    // 💡 JARからデフォルトの server.properties を抽出しようと試みる
    if let Ok(content) = JarExtractor::extract_to_string(jar_path, "server.properties") {
        props.push_str("\n# Default from JAR\n");
        props.push_str(&content);
    }
    
    fs::write(dir.join("server.properties"), props).await?;

    // 2. Paper/Spigot/Purpur 固有の Velocity 連携設定
    let spigot_yml = dir.join("spigot.yml");
    let paper_global = dir.join("config/paper-global.yml");

    // Spigot.yml の BungeeCord 有効化
    let s_content = String::from("settings:\n  bungeecord: true\n  sample-count: 12\n");
    fs::write(spigot_yml, s_content).await?;

    // Paper-global.yml の Velocity 連携設定
    if let Ok(_) = fs::create_dir_all(dir.join("config")).await {
        let mut writer = BufWriter::new(File::create(paper_global).await?);
        writer.write_all(b"proxies:\n  velocity:\n    enabled: true\n    online-mode: true\n    secret: ").await?;
        writer.write_all(secret.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }

    Ok(())
}
