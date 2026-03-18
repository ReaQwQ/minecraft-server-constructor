use serde::Deserialize;
use crate::fetcher::FetcherContext;
use crate::error::MsbError;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/**
 * 説明: GitHubのリリースからアセットを取得するFetcher
 */
pub struct GitHubFetcher {
    ctx: FetcherContext,
}

#[derive(Deserialize)]
struct GitHubRelease {
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

impl GitHubFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }

    pub fn ctx_clone(&self) -> FetcherContext {
        self.ctx.ctx_clone()
    }

    pub async fn get_latest_asset_url(&self, owner: &str, repo: &str, pattern: &str) -> Result<String, MsbError> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
        let mut req = self.ctx.client.get(&url);
        
        // 💡 回避策: 環境変数に GITHUB_TOKEN があれば認証ヘッダーを付けてレートリミットを上げる
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            req = req.bearer_auth(token);
        }

        let response = req.send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            
            // GitHub APIのレートリミット特有のエラーハンドリング
            if status.as_u16() == 403 && text.contains("rate limit") {
                return Err(MsbError::HttpError(format!(
                    "GitHub APIのレートリミット（回数制限）に到達しました。\n💡 解決策: しばらく待つか、環境変数 GITHUB_TOKEN を設定して再実行してください。\n詳細: {}", text
                )));
            }
            return Err(MsbError::HttpError(format!("GitHub APIエラー ({}): {}", status, text)));
        }

        let resp: GitHubRelease = response.json().await.map_err(|e| MsbError::ParseError(format!("JSON Parse Error: {}", e)))?;
        
        let pattern_lower = pattern.to_lowercase();
        resp.assets.into_iter()
            .find(|a| a.name.to_lowercase().contains(&pattern_lower))
            .map(|a| a.browser_download_url)
            .ok_or_else(|| MsbError::ParseError(format!("Asset matching '{pattern}' not found in {owner}/{repo}")))
    }

    /**
     * 説明: ストリーミング方式でファイルをダウンロードする (デコードエラー回避)
     */
    pub async fn download_jar(&self, url: &str, output_path: &Path) -> Result<(), MsbError> {
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
