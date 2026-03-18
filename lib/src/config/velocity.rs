use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::error::MsbError;

/// Velocityプロキシの設定ファイルを生成します。
/// 
/// @param dir 設定ファイルを生成するディレクトリのパス
/// @param secret 転送用シークレットキー
/// @requires 指定されたディレクトリが書き込み可能であること
/// @return 成功した場合はOk(()), 失敗した場合はMsbError
pub async fn create_velocity_config(dir: &Path, secret: &str) -> Result<(), MsbError> {
    let secret_path = dir.join("forwarding.secret");
    let mut fs = File::create(secret_path).await?;
    fs.write_all(secret.as_bytes()).await?;

    let toml_path = dir.join("velocity.toml");
    let content = r#"# Velocity Configuration
config-version = "2.7"
bind = "0.0.0.0:25565"
motd = "<aqua>Survival Server\n</aqua><gray>Powered by MSB/Velocity Proxy"
show-max-players = 100
online-mode = true
force-key-authentication = false
player-info-forwarding-mode = "modern"
forwarding-secret-file = "forwarding.secret"

[servers]
	backend = "127.0.0.1:25566"
	limbo = "127.0.0.1:25567"
	try = ["backend", "limbo"]

[advanced]
	compression-threshold = 256
"#;
    let mut ft = File::create(toml_path).await?;
    ft.write_all(content.as_bytes()).await?;

    Ok(())
}
