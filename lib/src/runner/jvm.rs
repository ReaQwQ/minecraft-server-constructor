use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use crate::error::MsbError;

/**
 * 説明: メモリ文字列（例: "8G", "512M"）をMB単位の数値に変換する
 * @param memory メモリ設定文字列
 * @return MB単位の数値
 */
pub fn parse_memory_to_mb(memory: &str) -> u32 {
    let memory = memory.to_uppercase();
    let numeric_part: String = memory.chars().filter(|c| c.is_digit(10)).collect();
    let value: u32 = numeric_part.parse().unwrap_or(4096);

    if memory.contains('G') {
        value * 1024
    } else {
        value
    }
}

/**
 * 説明: 全体メモリから各レイヤー（Backend, Proxy, Limbo）への配分を計算する
 * @param total_mb 全体のメモリ設定 (MB)
 * @return (Backend用, Proxy用, Limbo用) のメモリ設定文字列のタプル
 */
pub fn calculate_memory_distribution(total_mb: u32) -> (String, String, String) {
    let (p_mb, l_mb) = if total_mb >= 8192 {
        (1024, 128) 
    } else if total_mb >= 4096 {
        (512, 128)  
    } else {
        (256, 64)   
    };

    let b_mb = total_mb.saturating_sub(p_mb + l_mb).max(1024);

    (format!("{b_mb}M"), format!("{p_mb}M"), format!("{l_mb}M"))
}

/**
 * 説明: 各OS向けの起動スクリプト（run.bat, run.sh）を生成する。Aikar's Flags等の最適化にも対応。
 * @param dir スクリプトを生成するディレクトリ
 * @param jar_name 実行対象のJARファイル名
 * @param memory_config メモリ設定（例: "8G"）
 * @param java_path Java実行ファイルへのパス
 * @param use_aikar 最適化フラグ（Aikar's Flags）を使用するかどうか
 * @param platform ターゲットプラットフォーム
 * @requires tokio::fs::File, tokio::io::BufWriter
 * @return 成功時はOk(())、IO失敗時はMsbError
 */
pub async fn generate_startup_scripts(
    dir: &Path,
    jar_name: &str,
    memory_config: &str,
    java_path: Option<&Path>,
    use_aikar: bool,
    platform: &str,
) -> Result<(), MsbError> {
    let java_cmd = match java_path {
        Some(p) => p.to_string_lossy().to_string(),
        None => "java".to_string(),
    };

    let mut flags = format!("-Xmx{memory_config} -Xms{memory_config}");
    
    let mb = parse_memory_to_mb(memory_config);
    if use_aikar && mb >= 1024 {
        flags.push_str(" -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1");
    }

    if platform.to_lowercase() == "windows" {
        let bat_path = dir.join("run.bat");
        let bat_content = format!(
            "@echo off\nchcp 65001 > nul\nset \"JAVA_EXE={java_cmd}\"\n\"%JAVA_EXE%\" {flags} -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true -jar {jar_name} nogui\npause\n"
        );
        let bat_file = File::create(bat_path).await?;
        let mut writer = BufWriter::new(bat_file);
        writer.write_all(bat_content.as_bytes()).await?;
        writer.flush().await?;
    } else {
        let sh_path = dir.join("run.sh");
        let sh_content = format!(
            "#!/bin/bash\nJAVA_EXE=\"{java_cmd}\"\n\"$JAVA_EXE\" {flags} -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true -jar {jar_name} nogui\n"
        );
        let sh_file = File::create(&sh_path).await?;
        let mut writer = BufWriter::new(sh_file);
        writer.write_all(sh_content.as_bytes()).await?;
        writer.flush().await?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&sh_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&sh_path, perms);
            }
        }
    }

    Ok(())
}
