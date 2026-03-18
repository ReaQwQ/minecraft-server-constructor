use msb_core::{FetcherContext, PluginFetcher, ProtocolVersion, SpigotPlugin};
use crate::commands::deploy::get_fetcher_internal;

/**
 * 説明: 指定したエンジンの利用可能なバージョン一覧を取得する
 * @param engine サーバーエンジンの種類
 * @return バージョン文字列のリスト
 */
#[tauri::command]
pub async fn get_versions(engine: String) -> Result<Vec<String>, String> {
    if engine == "none" { return Ok(Vec::new()); }
    let ctx = FetcherContext::new();
    let fetcher = get_fetcher_internal(&engine, ctx);
    fetcher.list_versions().await.map_err(|e| e.to_string())
}

/**
 * 説明: 指定したエンジンとバージョンの利用可能なビルド一覧を取得する
 * @param engine サーバーエンジンの種類
 * @param version マインクラフトのバージョン
 * @return ビルド番号（または識別子）のリスト
 */
#[tauri::command]
pub async fn get_builds(engine: String, version: String) -> Result<Vec<String>, String> {
    if engine == "none" || version.is_empty() { return Ok(Vec::new()); }
    let ctx = FetcherContext::new();
    let fetcher = get_fetcher_internal(&engine, ctx);
    fetcher.list_builds(&version).await.map_err(|e| e.to_string())
}

/**
 * 説明: 利用可能なプロトコルバージョン（ViaVersion互換）の一覧を取得する
 * @return ProtocolVersion構造体のリスト
 */
#[tauri::command]
pub async fn get_protocol_list() -> Result<Vec<ProtocolVersion>, String> {
    let ctx = FetcherContext::new();
    let fetcher = PluginFetcher::with_context(ctx);
    fetcher.get_protocol_versions().await.map_err(|e| e.to_string())
}

/**
 * 説明: SpigotMCからプラグインの一覧をページネーションで取得する
 * @param page ページ番号
 * @param size 1ページあたりの取得数
 * @return SpigotPlugin構造体のリスト
 */
#[tauri::command]
pub async fn list_spigot_plugins(page: u32, size: u32) -> Result<Vec<SpigotPlugin>, String> {
    let ctx = FetcherContext::new();
    let fetcher = PluginFetcher::with_context(ctx);
    fetcher.list_spigot_plugins(page, size).await.map_err(|e| e.to_string())
}

/**
 * 説明: SpigotMCから特定のプラグインをダウンロードして指定フォルダに保存する
 * @param id プラグインID
 * @param name プラグイン名（ファイル名に使用）
 * @param output_dir 保存先ディレクトリ
 * @return 成功メッセージまたはエラー
 */
#[tauri::command]
pub async fn install_plugin(id: u32, name: String, output_dir: String) -> Result<String, String> {
    let ctx = FetcherContext::new();
    let fetcher = PluginFetcher::with_context(ctx);
    
    let download_url = format!("https://api.spiget.org/v2/resources/{}/download", id);
    let target_dir = if output_dir.is_empty() {
        std::env::current_dir().unwrap().join("plugins")
    } else {
        std::path::PathBuf::from(output_dir).join("plugins")
    };

    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }

    let file_path = target_dir.join(format!("{}.jar", name.replace(' ', "_")));
    fetcher.download_jar(&download_url, &file_path).await.map_err(|e| e.to_string())?;

    Ok(format!("Installed {} to {}", name, file_path.display()))
}
