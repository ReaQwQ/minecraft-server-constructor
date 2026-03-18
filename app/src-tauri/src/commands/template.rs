use base64::{Engine as _, engine::general_purpose};
use msb_core::DeployConfig;
use crate::commands::types::GuiDeployRequest;

/**
 * 説明: 現在の設定をテンプレートとしてエクスポート（Base64エンコードされたバイナリ）する
 * @param req デプロイ要求設定
 * @requires msb_core::DeployConfig
 * @return Base64エンコードされたmsbtデータ
 */
#[tauri::command]
pub async fn export_template(req: GuiDeployRequest) -> Result<String, String> {
    let config: DeployConfig = req.into();
    let binary = config.export_to_msbt().map_err(|e| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(binary))
}

/**
 * 説明: インポートされたBase64データから設定を復元する
 * @param data_base64 Base64エンコードされたmsbtデータ
 * @requires msb_core::DeployConfig
 * @return デプロイ要求設定構造体
 */
#[tauri::command]
pub async fn import_template(data_base64: String) -> Result<GuiDeployRequest, String> {
    let binary = general_purpose::STANDARD.decode(data_base64).map_err(|e| e.to_string())?;
    let config = DeployConfig::import_from_msbt(&binary).map_err(|e| e.to_string())?;
    
    Ok(GuiDeployRequest {
        server_type: config.server_type,
        version: config.version.unwrap_or_default(),
        build: config.build.unwrap_or_default(),
        proxy: config.proxy,
        proxy_version: config.proxy_version.unwrap_or_default(),
        proxy_build: config.proxy_build.unwrap_or_default(),
        memory: format!("{}M", config.memory_mb),
        memory_allocater: config.memory_allocater,
        jdk_auto: config.jdk_auto,
        enable_bedrock: config.enable_bedrock,
        enable_discord: config.enable_discord,
        enable_limbo: config.enable_limbo,
        enable_sonar: config.enable_sonar,
        accept_eula: config.accept_eula,
        protocol_range: config.protocol_range.unwrap_or_default(),
        platform: config.platform,
        output_dir: "".to_string(),
    })
}

/**
 * 説明: プリセット済みの公式テンプレートを取得する
 * @param name テンプレート名 (lobby, survival, modded)
 * @return デプロイ要求設定構造体
 */
#[tauri::command]
pub async fn get_official_template(name: String) -> Result<GuiDeployRequest, String> {
    let config = DeployConfig::get_official_template(&name).ok_or("Template not found")?;
    
    Ok(GuiDeployRequest {
        server_type: config.server_type,
        version: config.version.unwrap_or_default(),
        build: config.build.unwrap_or_default(),
        proxy: config.proxy,
        proxy_version: config.proxy_version.unwrap_or_default(),
        proxy_build: config.proxy_build.unwrap_or_default(),
        memory: format!("{}M", config.memory_mb),
        memory_allocater: config.memory_allocater,
        jdk_auto: config.jdk_auto,
        enable_bedrock: config.enable_bedrock,
        enable_discord: config.enable_discord,
        enable_limbo: config.enable_limbo,
        enable_sonar: config.enable_sonar,
        accept_eula: config.accept_eula,
        protocol_range: config.protocol_range.unwrap_or_default(),
        platform: config.platform,
        output_dir: "".to_string(),
    })
}
