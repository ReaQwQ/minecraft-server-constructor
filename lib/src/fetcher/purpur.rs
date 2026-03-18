use serde::Deserialize;
use crate::fetcher::{ServerFetcher, FetcherContext};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: PurpurMC APIを使用してサーバーJARを取得するFetcher実装
 * @requires reqwest, serde, tokio
 */
pub struct PurpurFetcher {
    ctx: FetcherContext,
}

#[derive(Deserialize)]
struct PurpurVersions {
    versions: Vec<String>,
}

#[derive(Deserialize)]
struct PurpurBuilds {
    builds: PurpurBuildList,
}

#[derive(Deserialize)]
struct PurpurBuildList {
    all: Vec<String>,
}

impl PurpurFetcher {
    /**
     * 説明: デフォルト設定で新しいFetcherを生成する
     * @return PurpurFetcherインスタンス
     */
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    /**
     * 説明: 既存の共有コンテキストを使用してFetcherを生成する
     * @param ctx 共有コンテキスト
     * @return PurpurFetcherインスタンス
     */
    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl ServerFetcher for PurpurFetcher {
    /**
     * 説明: 利用可能な全てのバージョンを降順で取得する
     */
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        if let Some(cached) = self.ctx.cache.get_versions("purpur").await {
            return Ok(cached);
        }
        let resp: PurpurVersions = self.ctx.client.get("https://api.purpurmc.org/v2/purpur").send().await?.json().await?;
        let res: Vec<String> = resp.versions.into_iter().rev().collect();
        self.ctx.cache.set_versions("purpur".to_string(), res.clone()).await;
        Ok(res)
    }

    /**
     * 説明: 指定したバージョンのビルド番号一覧を取得する
     */
    async fn list_builds(&self, version: &str) -> Result<Vec<String>, MsbError> {
        let cache_key = format!("purpur-{}", version);
        if let Some(cached) = self.ctx.cache.get_builds(&cache_key).await {
            return Ok(cached);
        }
        let url = format!("https://api.purpurmc.org/v2/purpur/{}", version);
        let resp: PurpurBuilds = self.ctx.client.get(&url).send().await?.json().await?;
        let res: Vec<String> = resp.builds.all.into_iter().rev().collect();
        self.ctx.cache.set_builds(cache_key, res.clone()).await;
        Ok(res)
    }

    async fn get_latest_version(&self) -> Result<String, MsbError> {
        let versions = self.list_versions().await?;
        versions.first().cloned().ok_or_else(|| MsbError::ParseError("No versions found".to_string()))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        Ok(format!("https://api.purpurmc.org/v2/purpur/{}/latest/download", version))
    }

    async fn get_specific_build_url(&self, version: &str, build: u32) -> Result<String, MsbError> {
        Ok(format!("https://api.purpurmc.org/v2/purpur/{}/{}/download", version, build))
    }

    /**
     * 説明: 指定されたURLからJARファイルをダウンロードし、非同期ストリームで書き込む
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
