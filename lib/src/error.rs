use thiserror::Error;
use std::sync::Arc;

/// アプリケーション全体で使用されるエラー型を表します。
/// 
/// @param なし
/// @requires なし
/// @return なし
#[derive(Error, Debug, Clone)]
pub enum MsbError {
    #[error("ネットワークエラー: {0}")]
    NetworkError(String),

    #[error("HTTPエラー: {0}")]
    HttpError(String),

    #[error("解析失敗: {0}")]
    ParseError(String),

    #[error("バージョン未検出: {0}")]
    VersionNotFound(String),

    #[error("I/Oエラー: {0}")]
    IoError(Arc<std::io::Error>),

    #[error("並列タスク失敗: {0}")]
    TaskError(String),

    #[error("不明なエラー: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for MsbError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e.to_string())
    }
}

impl From<std::io::Error> for MsbError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(Arc::new(e))
    }
}

impl From<serde_json::Error> for MsbError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}
