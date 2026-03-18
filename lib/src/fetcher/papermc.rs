use serde::Deserialize;
use crate::fetcher::{ServerFetcher, FetcherContext};
use crate::error::MsbError;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: PaperMC APIを使用してサーバーJARを取得するFetcher実装
 * @requires reqwest, serde, tokio
 */
pub struct PaperFetcher {
    ctx: FetcherContext,
    project: String,
}

#[derive(Deserialize)]
struct PaperVersionResponse {
    versions: Vec<String>,
}

#[derive(Deserialize)]
struct PaperBuildsResponse {
    builds: Vec<u32>,
}

#[derive(Deserialize)]
struct PaperBuildDetail {
    downloads: PaperDownloads,
}

#[derive(Deserialize)]
struct PaperDownloads {
    application: PaperApplication,
}

#[derive(Deserialize)]
struct PaperApplication {
    name: String,
}

impl PaperFetcher {
    /**
     * 説明: 指定されたプロジェクト名で新しいFetcherを生成する
     * @param project プロジェクト名 (paper, velocity, etc.)
     * @return PaperFetcherインスタンス
     */
    pub fn new(project: &str) -> Self {
        Self {
            ctx: FetcherContext::new(),
            project: project.to_string(),
        }
    }

    /**
     * 説明: 既存の共有コンテキストを使用してFetcherを生成する
     * @param ctx 共有コンテキスト
     * @param project プロジェクト名
     * @return PaperFetcherインスタンス
     */
    pub fn with_context(ctx: FetcherContext, project: &str) -> Self {
        Self {
            ctx,
            project: project.to_string(),
        }
    }

    /**
     * 説明: 指定したバージョンのビルド番号一覧を取得する（内部キャッシュ優先）
     * @param version マイクラバージョン
     * @return ビルド番号のリスト
     */
    pub async fn list_builds(&self, version: &str) -> Result<Vec<String>, MsbError> {
        let cache_key = format!("{}-{}", self.project, version);
        if let Some(cached) = self.ctx.cache.get_builds(&cache_key).await {
            return Ok(cached);
        }

        let builds_url = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            self.project, version
        );
        let builds_resp: PaperBuildsResponse = self.ctx.client.get(&builds_url).send().await?.json().await?;
        let res: Vec<String> = builds_resp.builds.into_iter().rev().map(|b| b.to_string()).collect();
        self.ctx.cache.set_builds(cache_key, res.clone()).await;
        Ok(res)
    }
}

#[async_trait]
impl ServerFetcher for PaperFetcher {
    /**
     * 説明: 利用可能な全てのバージョンを降順で取得する
     * @return バージョン文字列のリスト
     */
    async fn list_versions(&self) -> Result<Vec<String>, MsbError> {
        if let Some(cached) = self.ctx.cache.get_versions(&self.project).await {
            return Ok(cached);
        }

        let url = format!("https://api.papermc.io/v2/projects/{}", self.project);
        let resp: PaperVersionResponse = self.ctx.client.get(&url).send().await?.json().await?;
        let res: Vec<String> = resp.versions.into_iter().rev().collect();
        self.ctx.cache.set_versions(self.project.clone(), res.clone()).await;
        Ok(res)
    }

    async fn list_builds(&self, version: &str) -> Result<Vec<String>, MsbError> {
        self.list_builds(version).await
    }

    /**
     * 説明: 最新の安定版バージョン文字列を取得する
     */
    async fn get_latest_version(&self) -> Result<String, MsbError> {
        let versions = self.list_versions().await?;
        versions.first().cloned().ok_or_else(|| MsbError::ParseError("No versions found".to_string()))
    }

    /**
     * 説明: 最新ビルドのダウンロードURLを生成する
     */
    async fn get_download_url(&self, version: &str) -> Result<String, MsbError> {
        let builds = self.list_builds(version).await?;
        let latest_build = builds.first().ok_or(MsbError::VersionNotFound(version.to_string()))?;
        self.get_specific_build_url(version, latest_build.parse().unwrap()).await
    }

    /**
     * 説明: 指定されたバージョンの特定のビルドのダウンロードURLを取得する
     */
    async fn get_specific_build_url(&self, version: &str, build: u32) -> Result<String, MsbError> {
        let detail_url = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
            self.project, version, build
        );
        let detail_resp: PaperBuildDetail = self.ctx.client.get(&detail_url).send().await?.json().await?;
        
        Ok(format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}/downloads/{}",
            self.project, version, build, detail_resp.downloads.application.name
        ))
    }

    /**
     * 説明: 指定されたURLからJARファイルをダウンロードし、非同期ストリームで書き込む
     * @param url ダウンロードURL
     * @param output_path 保存先パス
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
