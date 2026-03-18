use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use msb_core::{DeployConfig, parse_memory_to_mb};

/**
 * 説明: フロントエンドからのデプロイ要求を格納する構造体
 */
#[derive(Deserialize, Serialize, Clone)]
pub struct GuiDeployRequest {
    pub server_type: String,
    pub version: String,
    pub build: String,
    pub proxy: String,
    pub proxy_version: String,
    pub proxy_build: String,
    pub memory: String,
    pub memory_allocater: bool,
    pub jdk_auto: bool,
    pub enable_bedrock: bool,
    pub enable_discord: bool,
    pub enable_limbo: bool,
    pub enable_sonar: bool,
    pub accept_eula: bool,
    pub protocol_range: String,
    pub platform: String,
    pub output_dir: String,
}

impl From<GuiDeployRequest> for DeployConfig {
    /**
     * 説明: GUIの要求形式をコアライブラリの設定形式に変換する
     * @param req GUIデプロイ要求
     * @return 変換後のDeployConfig
     */
    fn from(req: GuiDeployRequest) -> Self {
        Self {
            server_type: req.server_type,
            version: if req.version == "latest" || req.version.is_empty() { None } else { Some(req.version) },
            build: if req.build == "latest" || req.build.is_empty() { None } else { Some(req.build) },
            proxy: req.proxy,
            proxy_version: if req.proxy_version == "latest" || req.proxy_version.is_empty() { None } else { Some(req.proxy_version) },
            proxy_build: if req.proxy_build == "latest" || req.proxy_build.is_empty() { None } else { Some(req.proxy_build) },
            memory_mb: parse_memory_to_mb(&req.memory),
            accept_eula: req.accept_eula,
            enable_bedrock: req.enable_bedrock,
            enable_discord: req.enable_discord,
            enable_limbo: req.enable_limbo,
            enable_sonar: req.enable_sonar,
            protocol_range: if req.protocol_range.is_empty() { None } else { Some(req.protocol_range) },
            jdk_auto: req.jdk_auto,
            memory_allocater: req.memory_allocater,
            concurrently: true,
            platform: req.platform,
            base_dir: PathBuf::from("."),
        }
    }
}

/**
 * 説明: 生成されたフォルダの情報を格納する
 */
#[derive(Serialize, Deserialize)]
pub struct GeneratedFolder {
    pub name: String,
    pub path: String,
    pub created_at: String,
}

/**
 * 説明: ファイルの基本情報を格納する
 */
#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}

/**
 * 説明: システムリソースの使用統計
 */
#[derive(Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub cluster_count: usize,
}
