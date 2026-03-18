mod commands;

use commands::{
    deploy::deploy_cluster,
    fetch::{get_versions, get_builds, get_protocol_list, list_spigot_plugins, install_plugin},
    system::get_system_stats,
    template::{export_template, import_template, get_official_template},
    browse::{list_generated_folders, list_files_in_folder},
};

/**
 * 説明: Tauriアプリケーションのエントリーポイントおよびコマンドの登録を行う
 * @requires tauri, tauri_plugin_opener, tauri_plugin_dialog
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // Browse commands
            list_generated_folders, 
            list_files_in_folder, 
            
            // Fetch commands
            get_versions, 
            get_builds, 
            get_protocol_list, 
            list_spigot_plugins, 
            install_plugin,
            
            // System commands
            get_system_stats, 
            
            // Core logic commands
            deploy_cluster, 
            
            // Template commands
            export_template, 
            import_template, 
            get_official_template
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
