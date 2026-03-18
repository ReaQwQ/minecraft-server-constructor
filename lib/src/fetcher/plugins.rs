use crate::fetcher::FetcherContext;
use crate::error::MsbError;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolVersion {
    #[serde(rename = "minecraftVersion")]
    pub minecraft_version: String,
    pub version: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpigotPlugin {
    pub id: u32,
    pub name: String,
    pub tag: String,
    #[serde(rename = "icon")]
    pub icon_data: Option<SpigotIcon>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpigotIcon {
    pub url: String,
}

/**
 * 説明: 外部プラグイン（Spigot, Geyser等）を取得するFetcher
 */
pub struct PluginFetcher {
    ctx: FetcherContext,
}

impl PluginFetcher {
    pub fn new() -> Self {
        Self { ctx: FetcherContext::new() }
    }

    pub fn with_context(ctx: FetcherContext) -> Self {
        Self { ctx }
    }

    pub async fn get_geyser_url(&self, project: &str, platform: &str) -> Result<String, MsbError> {
        let url = format!("https://download.geysermc.org/v2/projects/{project}/versions/latest/builds/latest");
        Ok(format!("{url}/downloads/{platform}"))
    }

    /**
     * 説明: SpigotMC (SpiGet API) からプラグイン一覧を取得する (ページング対応)
     */
    pub async fn list_spigot_plugins(&self, page: u32, size: u32) -> Result<Vec<SpigotPlugin>, MsbError> {
        let url = format!("https://api.spiget.org/v2/resources?size={size}&page={page}&sort=-downloads&fields=id,name,tag,icon");
        let resp: Vec<SpigotPlugin> = self.ctx.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    pub async fn get_protocol_versions(&self) -> Result<Vec<ProtocolVersion>, MsbError> {
        let url = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/pc/common/protocolVersions.json";
        let resp: Vec<ProtocolVersion> = self.ctx.client.get(url).send().await?.json().await?;
        let mut filtered: Vec<ProtocolVersion> = resp.into_iter()
            .filter(|p| !p.minecraft_version.contains('w') && !p.minecraft_version.contains('-'))
            .collect();
        filtered.sort_by(|a, b| b.version.cmp(&a.version));
        Ok(filtered)
    }

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
