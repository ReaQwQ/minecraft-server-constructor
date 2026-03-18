use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind};
use tokio::fs;
use crate::commands::types::SystemStats;

/**
 * 説明: システムのリソース使用状況（CPU, メモリ）と生成済みクラスター数を取得する
 * @return SystemStats構造体
 */
#[tauri::command]
pub async fn get_system_stats() -> Result<SystemStats, String> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );
    sys.refresh_all();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    
    let mut cluster_count = 0;
    if let Ok(mut entries) = fs::read_dir("generated").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().is_dir() {
                cluster_count += 1;
            }
        }
    }
    
    Ok(SystemStats {
        cpu_usage,
        memory_total,
        memory_used,
        cluster_count,
    })
}
