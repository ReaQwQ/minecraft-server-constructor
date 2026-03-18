use serde::Deserialize;
use crate::fetcher::{ServerFetcher, FetcherContext, github::GitHubFetcher};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;

/**
 * 説明: Arclight (Forge/Fabric/NeoForge ハイブリッドサーバー) を取得するFetcher
 */
pub struct ArclightFetcher {
    ctx: FetcherContext,
    loader: String, // "forge", "fabric", "neoforge"
}

impl ArclightFetcher {
    pub fn new(loader: &str) -> Self {
        Self { 
            ctx: FetcherContext::new(),
            loader: loader.to_string(),
        }
    }

    pub fn with_context(ctx: FetcherContext, loader: &str) -> Self {
        Self { 
            ctx,
            loader: loader.to_string(),
        }
    }
}

#[async_trait]
impl ServerFetcher for ArclightFetcher {
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        // GitHub APIからリリース一覧を取得してタグ名を返す (簡易実装)
        let url = "https://api.github.com/repos/IzzelAliz/Arclight/releases";
        let mut req = self.ctx.client.get(url);
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            req = req.bearer_auth(token);
        }
        
        #[derive(Deserialize)]
        struct Release { tag_name: String }
        
        let resp = req.send().await?;
        if !resp.status().is_success() {
            return Err(MsbError::HttpError(format!("Arclight API Error: {}", resp.status())));
        }
        
        let releases: Vec<Release> = resp.json().await.map_err(|e| MsbError::ParseError(e.to_string()))?;
        Ok(releases.into_iter().map(|r| r.tag_name).collect())
    }

    async fn get_latest_version(&self) -> Result<String, MsbError> {
        // "latest" として扱う
        Ok("latest".to_string())
    }

    async fn get_download_url(&self, _version: &str) -> Result<String, MsbError> {
        let github = GitHubFetcher::with_context(self.ctx.ctx_clone());
        // arclight-forge-, arclight-fabric- などのパターンで検索
        let pattern = format!("arclight-{}-", self.loader);
        github.get_latest_asset_url("IzzelAliz", "Arclight", &pattern).await
    }

    async fn get_specific_build_url(&self, version: &str, _build: u32) -> Result<String, MsbError> {
        self.get_download_url(version).await
    }

    async fn download_jar(&self, url: &str, output_path: &Path) -> Result<(), MsbError> {
        let github = GitHubFetcher::with_context(self.ctx.ctx_clone());
        github.download_jar(url, output_path).await
    }
}
