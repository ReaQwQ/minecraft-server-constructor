/// 説明: MSB (Minecraft Server Builder) のコアライブラリ
pub mod error;
/// 説明: サーバーファイルの取得を行うモジュール群
pub mod fetcher;
/// 説明: 設定ファイルの生成・管理を行うモジュール群
pub mod config;
/// 説明: プロセスの実行管理を行うモジュール群
pub mod runner;
/// 説明: 高度な並列構築を統括するオーケストレーター
pub mod orchestrator;

pub use error::MsbError;
pub use fetcher::{
    Fetcher, FetcherContext, ServerFetcher,
    papermc::PaperFetcher, 
    purpur::PurpurFetcher, 
    fabric::FabricFetcher,
    vanilla::VanillaFetcher,
    github::GitHubFetcher,
    bds::BdsFetcher,
    arclight::ArclightFetcher,
    plugins::{PluginFetcher, ProtocolVersion, SpigotPlugin}, // 💡 追加
    jdk::JdkFetcher,
    jar::JarExtractor,
};
pub use config::eula::create_eula_file;
pub use config::velocity::create_velocity_config;
pub use config::backend::optimize_backend_config;
pub use config::limbo::create_limbo_config;
pub use runner::jvm::{generate_startup_scripts, calculate_memory_distribution, parse_memory_to_mb};
pub use orchestrator::{run_orchestration, DeployConfig};
