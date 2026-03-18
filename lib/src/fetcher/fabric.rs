use serde::Deserialize;
use crate::fetcher::{ServerFetcher, FetcherContext};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: FabricのサーバーJARを取得するFetcher
 */
pub struct FabricFetcher {
    ctx: FetcherContext,
}

#[derive(Deserialize)]
struct FabricGameVersion {
    version: String,
    stable: bool,
}

#[derive(Deserialize)]
struct FabricLoaderVersion {
    loader: FabricLoader,
}

#[derive(Deserialize)]
struct FabricLoader {
    version: String,
}

impl FabricFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl ServerFetcher for FabricFetcher {
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        let resp: Vec<FabricGameVersion> = self.ctx.client.get("https://meta.fabricmc.net/v2/versions/game").send().await?.json().await?;
        Ok(resp.into_iter().map(|v| v.version).collect())
    }

    async fn get_latest_version(&self) -> Result<String, MsbError> {
        let resp: Vec<FabricGameVersion> = self.ctx.client.get("https://meta.fabricmc.net/v2/versions/game").send().await?.json().await?;
        resp.into_iter()
            .find(|v| v.stable)
            .map(|v| v.version)
            .ok_or_else(|| MsbError::ParseError("No stable versions found".to_string()))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        let loaders: Vec<FabricLoaderVersion> = self.ctx.client.get("https://meta.fabricmc.net/v2/versions/loader").send().await?.json().await?;
        let loader_version = loaders.first().ok_or_else(|| MsbError::ParseError("No loader found".to_string()))?.loader.version.as_str();
        
        Ok(format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/server/jar", version, loader_version))
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
