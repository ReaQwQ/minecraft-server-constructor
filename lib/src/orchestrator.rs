use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::task::JoinSet;
use crate::{
    Fetcher, FetcherContext, PaperFetcher, PurpurFetcher, FabricFetcher, VanillaFetcher, 
    GitHubFetcher, JdkFetcher, PluginFetcher, JarExtractor, ServerFetcher, ArclightFetcher,
    BdsFetcher, generate_startup_scripts, calculate_memory_distribution,
    create_eula_file, optimize_backend_config, MsbError
};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};

/**
 * 説明: クラスター構築の設定を保持する構造体
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeployConfig {
    pub server_type: String,
    pub version: Option<String>,
    pub build: Option<String>,
    pub proxy: String,
    pub proxy_version: Option<String>,
    pub proxy_build: Option<String>,
    pub memory_mb: u32,
    pub accept_eula: bool,
    pub enable_bedrock: bool,
    pub enable_discord: bool,
    pub enable_limbo: bool,
    pub enable_sonar: bool,
    pub protocol_range: Option<String>,
    pub jdk_auto: bool,
    pub memory_allocater: bool,
    pub concurrently: bool,
    pub platform: String,
    pub base_dir: PathBuf,
}

impl DeployConfig {
    /**
     * 説明: 設定をバイナリシリアライズ形式 (.msbt) でエクスポートする
     * @return シリアライズされたバイト列
     */
    pub fn export_to_msbt(&self) -> Result<Vec<u8>, MsbError> {
        bincode::serialize(self).map_err(|e| MsbError::Unknown(format!("Export failed: {}", e)))
    }

    /**
     * 説明: バイナリデータから設定をインポートする
     * @param data バイト列
     * @return 復元されたDeployConfig
     */
    pub fn import_from_msbt(data: &[u8]) -> Result<Self, MsbError> {
        bincode::deserialize(data).map_err(|e| MsbError::Unknown(format!("Import failed: {}", e)))
    }

    /**
     * 説明: 指定された名前の公式テンプレート設定を生成する
     * @param name テンプレート名
     * @return 設定のオプション
     */
    pub fn get_official_template(name: &str) -> Option<Self> {
        let mut base = Self {
            server_type: "purpur".to_string(),
            version: Some("1.21.1".to_string()),
            build: Some("latest".to_string()),
            proxy: "velocity".to_string(),
            proxy_version: Some("latest".to_string()),
            proxy_build: Some("latest".to_string()),
            memory_mb: 8192,
            accept_eula: true,
            enable_bedrock: true,
            enable_discord: false,
            enable_limbo: true,
            enable_sonar: true,
            protocol_range: Some("754-latest".to_string()),
            jdk_auto: true,
            memory_allocater: true,
            concurrently: true,
            platform: "windows".to_string(),
            base_dir: PathBuf::from("."),
        };

        match name {
            "lobby" => {
                base.server_type = "paper".to_string();
                base.memory_mb = 4096;
                base.enable_discord = true;
                Some(base)
            },
            "survival" => {
                base.server_type = "purpur".to_string();
                base.memory_mb = 12288;
                Some(base)
            },
            "modded" => {
                base.server_type = "arclight-forge".to_string();
                base.memory_mb = 16384;
                base.enable_sonar = false;
                Some(base)
            },
            _ => None
        }
    }
}

/**
 * 説明: 安全な暗号学的乱数を用いて秘密鍵を生成する
 * @param length 長さ
 * @return ランダムなアルファベット文字列
 */
fn generate_random_secret(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/**
 * 説明: 複数のサーバーコンポーネントを並列に構築・配置するオーケストレーター
 * @param config デプロイ設定
 * @param ctx フェッチャーコンテキスト
 * @requires tokio, msb_core::fetcher, msb_core::config
 * @return デプロイの成否
 */
pub async fn run_orchestration(config: DeployConfig, ctx: FetcherContext) -> Result<(), MsbError> {
    let secret = Arc::new(generate_random_secret(32));
    let base_dir = Arc::new(config.base_dir.clone());
    
    let (b_mem, p_mem, l_mem) = if config.memory_allocater {
        calculate_memory_distribution(config.memory_mb)
    } else {
        (format!("{}M", config.memory_mb), "1G".to_string(), "128M".to_string())
    };

    let b_mem = Arc::new(b_mem);
    let p_mem = Arc::new(p_mem);
    let l_mem = Arc::new(l_mem);

    let backend_dir = base_dir.join("backend");
    let proxy_dir = base_dir.join("proxy");
    let limbo_dir = base_dir.join("limbo");
    let bin_dir = base_dir.join("bin");

    // ディレクトリの一括作成
    fs::create_dir_all(&backend_dir).await?;
    if config.proxy != "none" { fs::create_dir_all(&proxy_dir).await?; }
    if config.enable_limbo { fs::create_dir_all(&limbo_dir).await?; }
    fs::create_dir_all(&bin_dir).await?;

    let mut set = JoinSet::new();
    let java_exe = Arc::new(tokio::sync::Mutex::new(PathBuf::from("java")));

    // 1. JDK Task: Javaランタイムの自動取得
    if config.jdk_auto {
        set.spawn(setup_jdk_task(ctx.ctx_clone(), bin_dir.clone(), config.version.clone(), config.server_type.clone(), Arc::clone(&java_exe)));
    }

    // 2. Backend Task: メインサーバーのデプロイ
    set.spawn(setup_backend_task(ctx.ctx_clone(), backend_dir.clone(), config.server_type.clone(), config.version.clone(), config.build.clone(), Arc::clone(&secret), config.accept_eula));

    // 3. Proxy Task: プロキシサーバー（Velocity等）のデプロイ
    if config.proxy != "none" {
        set.spawn(setup_proxy_task(ctx.ctx_clone(), proxy_dir.clone(), config.proxy.clone(), config.proxy_version.clone(), config.proxy_build.clone(), Arc::clone(&secret)));
    }

    // 4. Limbo Task: 待機用サーバー（NanoLimbo）のデプロイ
    if config.enable_limbo {
        set.spawn(setup_limbo_task(ctx.ctx_clone(), limbo_dir.clone(), Arc::clone(&secret)));
    }

    // 5. Plugins Task: 各種プラグインの自動設定
    if config.enable_discord || config.enable_bedrock || config.enable_sonar || config.protocol_range.is_some() {
        set.spawn(setup_plugins_task(ctx.ctx_clone(), backend_dir.clone(), proxy_dir.clone(), config.proxy.clone(), config.protocol_range.clone(), config.enable_discord, config.enable_bedrock, config.enable_sonar));
    }

    // タスクの完了を待機
    while let Some(res) = set.join_next().await {
        res.map_err(|e| MsbError::TaskError(e.to_string()))??;
    }

    let final_java = java_exe.lock().await.clone();
    finalize_deployment(&config, &base_dir, &backend_dir, &proxy_dir, &limbo_dir, &b_mem, &p_mem, &l_mem, Some(&final_java)).await
}

/**
 * 説明: JDKのダウンロードと展開を担当するサブタスク
 */
async fn setup_jdk_task(ctx: FetcherContext, bin_dir: PathBuf, version: Option<String>, server_type: String, java_exe: Arc<tokio::sync::Mutex<PathBuf>>) -> Result<(), MsbError> {
    let jdks = JdkFetcher::with_context(ctx);
    let v = version.as_deref().unwrap_or("1.21.1");
    let j_ver = JdkFetcher::get_recommended_java_version(v, &server_type);
    let path = jdks.download_and_extract_jdk(j_ver, &bin_dir).await?;
    let mut lock = java_exe.lock().await;
    *lock = path;
    Ok(())
}

/**
 * 説明: バックエンドサーバーのJAR取得と初期設定を担当するサブタスク
 */
async fn setup_backend_task(ctx: FetcherContext, b_dir: PathBuf, st: String, version: Option<String>, build: Option<String>, secret: Arc<String>, accept_eula: bool) -> Result<(), MsbError> {
    let fetcher = get_fetcher_internal(&st, ctx);
    let v = match version { Some(v) => v, None => fetcher.get_latest_version().await? };
    
    let download_url = if let Some(b) = build {
        if b == "latest" { fetcher.get_download_url(&v).await? }
        else { fetcher.get_specific_build_url(&v, b.parse().unwrap_or(0)).await? }
    } else {
        fetcher.get_download_url(&v).await?
    };

    let jar_path = b_dir.join(format!("{}.jar", st));
    fetcher.download_jar(&download_url, &jar_path).await?;
    create_eula_file(&b_dir, accept_eula).await?;
    optimize_backend_config(&b_dir, &jar_path, &secret).await?;
    Ok(())
}

/**
 * 説明: プロキシサーバーのJAR取得と初期設定を担当するサブタスク
 */
async fn setup_proxy_task(ctx: FetcherContext, p_dir: PathBuf, pt: String, pv: Option<String>, pb: Option<String>, secret: Arc<String>) -> Result<(), MsbError> {
    let p_fetcher = PaperFetcher::with_context(ctx, &pt);
    let v = match pv { Some(v) => v, None => p_fetcher.get_latest_version().await? };
    
    let download_url = if let Some(b) = pb {
        if b == "latest" { p_fetcher.get_download_url(&v).await? }
        else { p_fetcher.get_specific_build_url(&v, b.parse().unwrap_or(0)).await? }
    } else {
        p_fetcher.get_download_url(&v).await?
    };

    let p_jar_name = format!("{}.jar", pt);
    let p_jar_path = p_dir.join(&p_jar_name);
    p_fetcher.download_jar(&download_url, &p_jar_path).await?;
    
    if pt == "velocity" {
        JarExtractor::extract_and_replace(&p_jar_path, "velocity.toml", &p_dir.join("velocity.toml"), vec![("player-info-forwarding-mode = \"none\"", "player-info-forwarding-mode = \"modern\"")])?;
        fs::write(p_dir.join("forwarding.secret"), secret.as_bytes()).await?;
    }
    Ok(())
}

/**
 * 説明: NanoLimboのJAR取得と設定を担当するサブタスク
 */
async fn setup_limbo_task(ctx: FetcherContext, l_dir: PathBuf, secret: Arc<String>) -> Result<(), MsbError> {
    let github = GitHubFetcher::with_context(ctx);
    let l_url = github.get_latest_asset_url("Nan1t", "NanoLimbo", "all.jar").await?;
    let l_jar_path = l_dir.join("nanolimbo.jar");
    github.download_jar(&l_url, &l_jar_path).await?;
    JarExtractor::extract_and_replace(&l_jar_path, "settings.yml", &l_dir.join("settings.yml"), vec![("type: NONE", "type: MODERN"), ("secret: ''", &format!("secret: '{}'", secret))])?;
    Ok(())
}

/**
 * 説明: 必要なプラグインの選定とダウンロードを担当するサブタスク
 */
async fn setup_plugins_task(ctx: FetcherContext, b_dir: PathBuf, p_dir: PathBuf, pt: String, protocol_range: Option<String>, enable_discord: bool, enable_bedrock: bool, enable_sonar: bool) -> Result<(), MsbError> {
    let github = GitHubFetcher::with_context(ctx.ctx_clone());
    let plugins = PluginFetcher::with_context(ctx.ctx_clone());
    
    let target_plugin_dir = if pt == "none" {
        let d = b_dir.join("plugins");
        fs::create_dir_all(&d).await?;
        d
    } else {
        let d = p_dir.join("plugins");
        fs::create_dir_all(&d).await?;
        d
    };
    
    if enable_discord {
        let d_dir = b_dir.join("plugins");
        fs::create_dir_all(&d_dir).await?;
        let d_url = github.get_latest_asset_url("DiscordSRV", "DiscordSRV", "DiscordSRV-Build").await?;
        github.download_jar(&d_url, &d_dir.join("DiscordSRV.jar")).await?;
    }
    
    if enable_bedrock {
        let platform = if pt == "none" { "spigot" } else { &pt };
        let g_url = plugins.get_geyser_url("geyser", platform).await?;
        let f_url = plugins.get_geyser_url("floodgate", platform).await?;
        let g_path = target_plugin_dir.join("Geyser.jar");
        let f_path = target_plugin_dir.join("Floodgate.jar");
        let (gr, fr) = tokio::join!(
            plugins.download_jar(&g_url, &g_path),
            plugins.download_jar(&f_url, &f_path)
        );
        gr?; fr?;
    }

    if enable_sonar {
        let asset_name = if pt == "velocity" { "Sonar-Velocity.jar" } else { "Sonar-Bungee.jar" };
        let s_url = github.get_latest_asset_url("jonesdevelopment", "sonar", asset_name).await?;
        github.download_jar(&s_url, &target_plugin_dir.join("Sonar.jar")).await?;
    }
    
    if let Some(v_s) = protocol_range {
        setup_viaversion_task(&github, &target_plugin_dir, &v_s).await?;
    }
    Ok(())
}

/**
 * 説明: ViaVersion関連のダウンロードとプロトコル制限設定を担当する
 */
async fn setup_viaversion_task(github: &GitHubFetcher, target_plugin_dir: &Path, v_s: &str) -> Result<(), MsbError> {
    let mut allowed_ids = std::collections::HashSet::new();
    let parts: Vec<String> = v_s.split(',').map(|s| s.trim().to_string()).collect();
    for part in parts {
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() == 2 {
                let min = bounds[0].parse::<u32>().unwrap_or(0);
                let max = if bounds[1].to_lowercase() == "latest" { 1000 } else { bounds[1].parse::<u32>().unwrap_or(1000) };
                for p in min..=max { allowed_ids.insert(p); }
            }
        } else if part.to_lowercase() == "latest" {
            allowed_ids.insert(1000);
        } else if let Ok(p) = part.parse::<u32>() {
            allowed_ids.insert(p);
        }
    }
    
    if !allowed_ids.is_empty() {
        let v_url = github.get_latest_asset_url("ViaVersion", "ViaVersion", "ViaVersion").await?;
        github.download_jar(&v_url, &target_plugin_dir.join("ViaVersion.jar")).await?;
        let vb_url = github.get_latest_asset_url("ViaVersion", "ViaBackwards", "ViaBackwards").await?;
        github.download_jar(&vb_url, &target_plugin_dir.join("ViaBackwards.jar")).await?;
        
        let min_p = allowed_ids.iter().min().copied().unwrap_or(1000);
        if min_p <= 47 {
            let vr_url = github.get_latest_asset_url("ViaVersion", "ViaRewind", "ViaRewind").await?;
            github.download_jar(&vr_url, &target_plugin_dir.join("ViaRewind.jar")).await?;
        }
        
        let v_config_dir = target_plugin_dir.join("ViaVersion");
        fs::create_dir_all(&v_config_dir).await?;
        let mut blocked = Vec::new();
        for p in 0..=1000 { if !allowed_ids.contains(&p) { blocked.push(p); } }
        let config_content = format!("block-protocols: {:?}\n", blocked);
        fs::write(v_config_dir.join("config.yml"), config_content).await?;
    }
    Ok(())
}

/**
 * 説明: 全コンポーネントの配置完了後、起動スクリプト等を生成して最終化する
 */
async fn finalize_deployment(config: &DeployConfig, base_dir: &Path, backend_dir: &Path, proxy_dir: &Path, limbo_dir: &Path, b_mem: &str, p_mem: &str, l_mem: &str, final_java: Option<&Path>) -> Result<(), MsbError> {
    let b_jar_name = format!("{}.jar", config.server_type);
    let p_jar_name = format!("{}.jar", config.proxy);

    generate_startup_scripts(backend_dir, &b_jar_name, b_mem, final_java, config.memory_allocater, &config.platform).await?;
    
    if config.proxy != "none" {
        generate_startup_scripts(proxy_dir, &p_jar_name, p_mem, final_java, config.memory_allocater, &config.platform).await?;
    }
    
    if config.enable_limbo {
        generate_startup_scripts(limbo_dir, "nanolimbo.jar", l_mem, final_java, config.memory_allocater, &config.platform).await?;
    }

    if config.platform.to_lowercase() == "windows" {
        write_windows_start_bat(config, base_dir).await?;
    } else {
        write_unix_start_sh(config, base_dir).await?;
    }

    Ok(())
}

/**
 * 説明: Windows環境向けの全体起動スクリプトを生成する
 */
async fn write_windows_start_bat(config: &DeployConfig, base_dir: &Path) -> Result<(), MsbError> {
    let mut start_bat = String::from("@echo off\nchcp 65001 > nul\n");
    if config.proxy != "none" {
        let limbo_part = if config.enable_limbo { " \"cd limbo && run.bat\"" } else { "" };
        if config.concurrently {
            start_bat.push_str(&format!("concurrently -n \"BACKEND,PROXY,LIMBO\" -c \"green,blue,magenta\" \"cd backend && run.bat\" \"cd proxy && run.bat\"{limbo_part}\n"));
        } else {
            if config.enable_limbo { start_bat.push_str("start \"Limbo\" /D \"limbo\" run.bat\n"); }
            start_bat.push_str("start \"Backend\" /D \"backend\" run.bat\nstart \"Proxy\" /D \"proxy\" run.bat\n");
        }
    } else {
        start_bat.push_str("cd backend && run.bat\n");
    }
    fs::write(base_dir.join("start.bat"), start_bat).await.map_err(Into::into)
}

/**
 * 説明: Unix/Linux環境向けの全体起動スクリプトを生成する
 */
async fn write_unix_start_sh(config: &DeployConfig, base_dir: &Path) -> Result<(), MsbError> {
    let mut start_sh = String::from("#!/bin/bash\n");
    if config.proxy != "none" {
        let limbo_part = if config.enable_limbo { " \"cd limbo && ./run.sh\"" } else { "" };
        if config.concurrently {
            start_sh.push_str(&format!("concurrently -n \"BACKEND,PROXY,LIMBO\" -c \"green,blue,magenta\" \"cd backend && ./run.sh\" \"cd proxy && ./run.sh\"{limbo_part}\n"));
        } else {
            if config.enable_limbo { start_sh.push_str("cd limbo && ./run.sh &\n"); }
            start_sh.push_str("cd backend && ./run.sh &\ncd ../proxy && ./run.sh &\nwait\n");
        }
    } else {
        start_sh.push_str("cd backend && ./run.sh\n");
    }
    let sh_path = base_dir.join("start.sh");
    fs::write(&sh_path, start_sh).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&sh_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&sh_path, perms).await?;
    }
    Ok(())
}

/**
 * 説明: サーバータイプ名から具体的なFetcherの実装を取得する
 */
fn get_fetcher_internal(server_type: &str, ctx: FetcherContext) -> Fetcher {
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
