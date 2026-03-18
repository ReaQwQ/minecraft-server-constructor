use std::path::PathBuf;
use chrono::Local;
use msb_core::{FetcherContext, run_orchestration, DeployConfig, Fetcher, PaperFetcher, PurpurFetcher, FabricFetcher, VanillaFetcher, BdsFetcher, ArclightFetcher};
use crate::commands::types::GuiDeployRequest;

/**
 * 説明: クラスターデプロイメントを実行するメインコマンド
 * @param req フロントエンドからのデプロイ要求設定
 * @requires msb_core::orchestrator
 * @return 成功時はデプロイ先のパス文字列、失敗時はエラーメッセージ
 */
#[tauri::command]
pub async fn deploy_cluster(req: GuiDeployRequest) -> Result<String, String> {
    let ctx = FetcherContext::new();
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    // 出力先ディレクトリの決定
    let base_dir = if req.output_dir.is_empty() { 
        std::env::current_dir().map_err(|e| e.to_string())?.join("generated").join(now) 
    } else { 
        PathBuf::from(&req.output_dir).join(now) 
    };
    
    let mut config: DeployConfig = req.into();
    config.base_dir = base_dir.clone();

    // オーケストレーションの実行
    run_orchestration(config, ctx).await.map_err(|e| e.to_string())?;
    
    Ok(base_dir.to_string_lossy().to_string())
}

/**
 * 説明: サーバータイプに応じた内部フェッチャーを生成する（ヘルパー関数）
 * @param server_type サーバーの種類
 * @param ctx フェッチャーコンテキスト
 * @return 共通Fetcher列挙型
 */
pub fn get_fetcher_internal(server_type: &str, ctx: FetcherContext) -> Fetcher {
    match server_type.to_lowercase().as_str() {
        "purpur" => Fetcher::Purpur(PurpurFetcher::with_context(ctx)),
        "fabric" => Fetcher::Fabric(FabricFetcher::with_context(ctx)),
        "bds" => Fetcher::Bds(BdsFetcher::with_context(ctx)),
        "vanilla" => Fetcher::Vanilla(VanillaFetcher::with_context(ctx)),
        "arclight" | "arclight-forge" => Fetcher::Arclight(ArclightFetcher::with_context(ctx, "forge")),
        "arclight-fabric" => Fetcher::Arclight(ArclightFetcher::with_context(ctx, "fabric")),
        "arclight-neoforge" => Fetcher::Arclight(ArclightFetcher::with_context(ctx, "neoforge")),
        _ => Fetcher::Paper(PaperFetcher::with_context(ctx, server_type)),
    }
}
