use std::path::{Path, PathBuf};
use tokio::fs;
use crate::commands::types::{GeneratedFolder, FileInfo};

/**
 * 説明: 生成済みのサーバーフォルダ一覧を取得する
 * @param base_path 基準パス（Noneの場合はデフォルトのgeneratedフォルダ）
 * @return GeneratedFolder構造体のリスト
 */
#[tauri::command]
pub async fn list_generated_folders(base_path: Option<String>) -> Result<Vec<GeneratedFolder>, String> {
    let mut folders = Vec::new();
    let generated_path = base_path.map(PathBuf::from).unwrap_or_else(|| Path::new("generated").to_path_buf());
    
    if !generated_path.exists() { return Ok(folders); }
    
    let mut entries = fs::read_dir(generated_path).await.map_err(|e| e.to_string())?;
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            folders.push(GeneratedFolder {
                name: name.clone(),
                path: path.to_string_lossy().to_string(),
                created_at: name,
            });
        }
    }
    
    folders.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(folders)
}

/**
 * 説明: 特定のフォルダ内のファイル一覧を取得する
 * @param path フォルダパス
 * @return FileInfo構造体のリスト
 */
#[tauri::command]
pub async fn list_files_in_folder(path: String) -> Result<Vec<FileInfo>, String> {
    let mut files = Vec::new();
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err("Directory not found".to_string());
    }
    
    let mut entries = fs::read_dir(dir_path).await.map_err(|e| e.to_string())?;
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let metadata = entry.metadata().await.map_err(|e| e.to_string())?;
        files.push(FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
        });
    }
    
    Ok(files)
}
