pub mod papermc;
pub mod purpur;
pub mod fabric;
pub mod vanilla;
pub mod github;
pub mod plugins;
pub mod bds;
pub mod jdk;
pub mod jar;
pub mod arclight;
pub mod cache;

use std::path::Path;
use std::sync::Arc;
use async_trait::async_trait;
use crate::error::MsbError;
use self::cache::FetchCache;

pub enum Fetcher {
    Paper(papermc::PaperFetcher),
    Purpur(purpur::PurpurFetcher),
    Fabric(fabric::FabricFetcher),
    Vanilla(vanilla::VanillaFetcher),
    Bds(bds::BdsFetcher),
    Arclight(arclight::ArclightFetcher),
}

impl Fetcher {
    pub async fn get_latest_version(&self) -> Result<String, MsbError> {
        match self {
            Self::Paper(f) => f.get_latest_version().await,
            Self::Purpur(f) => f.get_latest_version().await,
            Self::Fabric(f) => f.get_latest_version().await,
            Self::Vanilla(f) => f.get_latest_version().await,
            Self::Bds(f) => f.get_latest_version().await,
            Self::Arclight(f) => f.get_latest_version().await,
        }
    }

    pub async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        match self {
            Self::Paper(f) => f.list_versions().await,
            Self::Purpur(f) => f.list_versions().await,
            Self::Fabric(f) => f.list_versions().await,
            Self::Vanilla(f) => f.list_versions().await,
            Self::Bds(f) => f.list_versions().await,
            Self::Arclight(f) => f.list_versions().await,
        }
    }

    pub async fn list_builds(&self, version: &str) -> Result<Vec<String>, MsbError> {
        match self {
            Self::Paper(f) => f.list_builds(version).await,
            Self::Purpur(f) => f.list_builds(version).await,
            _ => Ok(vec!["latest".to_string()]),
        }
    }

    pub async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        match self {
            Self::Paper(f) => f.get_download_url(version).await,
            Self::Purpur(f) => f.get_download_url(version).await,
            Self::Fabric(f) => f.get_download_url(version).await,
            Self::Vanilla(f) => f.get_download_url(version).await,
            Self::Bds(f) => f.get_download_url(version).await,
            Self::Arclight(f) => f.get_download_url(version).await,
        }
    }

    pub async fn get_specific_build_url(&self, version: &str, build: u32) -> Result<String, MsbError> {
        match self {
            Self::Paper(f) => f.get_specific_build_url(version, build).await,
            Self::Purpur(f) => f.get_specific_build_url(version, build).await,
            _ => self.get_download_url(version).await,
        }
    }

    pub async fn download_jar(&self, url: &str, output_path: &Path) -> Result<(), MsbError> {
        match self {
            Self::Paper(f) => f.download_jar(url, output_path).await,
            Self::Purpur(f) => f.download_jar(url, output_path).await,
            Self::Fabric(f) => f.download_jar(url, output_path).await,
            Self::Vanilla(f) => f.download_jar(url, output_path).await,
            Self::Bds(f) => f.download_jar(url, output_path).await,
            Self::Arclight(f) => f.download_jar(url, output_path).await,
        }
    }
}

#[async_trait]
pub trait ServerFetcher: Send + Sync {
    async fn list_versions(&self) -> Result<Vec<String>, MsbError>;
    async fn list_builds(&self, version: &str) -> Result<Vec<String>, MsbError> {
        let _ = version;
        Ok(vec!["latest".to_string()])
    }
    async fn get_latest_version(&self) -> Result<String, MsbError>;
    async fn get_download_url(&self, version: &str) -> Result<String, MsbError>;
    async fn get_specific_build_url(&self, version: &str, build: u32) -> Result<String, MsbError>;
    async fn download_jar(&self, url: &str, output_path: &Path) -> Result<(), MsbError>;
}

/**
 * 💡 共有コンテキスト: 全フェッチャーで最強のクライアントを共有
 */
#[derive(Clone)]
pub struct FetcherContext {
    pub client: Arc<reqwest::Client>,
    pub cache: Arc<FetchCache>,
}

impl FetcherContext {
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::builder()
                .user_agent("MSB/3.2.6 (Absolute Decoder Edition)")
                .tcp_nodelay(true)
                .build()
                .expect("Failed to create context")),
            cache: Arc::new(FetchCache::new()),
        }
    }

    pub fn ctx_clone(&self) -> Self {
        Self { 
            client: Arc::clone(&self.client),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl Default for FetcherContext {
    fn default() -> Self {
        Self::new()
    }
}
