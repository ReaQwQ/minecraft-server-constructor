use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::error::MsbError;

/// Limboサーバーの設定ファイルを作成します。
/// 
/// @param dir 設定ファイルを生成するディレクトリのパス
/// @param secret Velocityのシークレットキー
/// @requires 指定されたディレクトリが書き込み可能であること
/// @return 成功した場合はOk(()), 失敗した場合はMsbError
pub async fn create_limbo_config(dir: &Path, secret: &str) -> Result<(), MsbError> {
    let config_path = dir.join("settings.yml");
    let content = format!(
r#"bind:
  ip: '127.0.0.1'
  port: 25567
maxPlayers: 100
ping:
  description: '{{"text": "Limbo Server"}}'
  version: 'Limbo'
  protocol: -1
dimension: THE_END
gameMode: 3
infoForwarding:
  type: MODERN
  secret: '{}'
netty:
  useEpoll: true
  threads:
    bossGroup: 1
    workerGroup: 4
"#, secret);
    let mut file = File::create(config_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}
