use crate::fetcher::{ServerFetcher, FetcherContext};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: Bedrock Dedicated Server (BDS) を取得するFetcher
 */
pub struct BdsFetcher {
    ctx: FetcherContext,
}

impl BdsFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl ServerFetcher for BdsFetcher {
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        Ok(vec!["latest".to_string()])
    }

    async fn get_latest_version(&self) -> Result<String, MsbError> {
        Ok("1.21.60.10".to_string())
    }

    async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        let v = if version == "latest" { "1.21.60.10" } else { version };
        Ok(format!("https://www.minecraft.net/bedrockdedicatedserver/bin-win/bedrock-server-{v}.zip"))
    }

    async fn get_specific_build_url(&self, _version: &str, _build: u32) -> Result<String, MsbError> {
        let v = self.get_latest_version().await?;
        self.get_download_url(&v).await
    }

    /**
     * 説明: ストリーミング方式でZIPをダウンロードする (デコードエラー回避)
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
