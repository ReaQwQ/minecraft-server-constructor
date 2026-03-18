use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/**
 * 説明: フェッチャーの結果をキャッシュするための構造体
 */
pub struct FetchCache {
    versions: RwLock<HashMap<String, (Vec<String>, DateTime<Utc>)>>,
    builds: RwLock<HashMap<String, (Vec<String>, DateTime<Utc>)>>,
}

impl FetchCache {
    pub fn new() -> Self {
        Self {
            versions: RwLock::new(HashMap::new()),
            builds: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_versions(&self, key: &str) -> Option<Vec<String>> {
        let read = self.versions.read().await;
        if let Some((vals, time)) = read.get(key) {
            if Utc::now() - *time < chrono::Duration::minutes(30) {
                return Some(vals.clone());
            }
        }
        None
    }

    pub async fn set_versions(&self, key: String, vals: Vec<String>) {
        let mut write = self.versions.write().await;
        write.insert(key, (vals, Utc::now()));
    }

    pub async fn get_builds(&self, key: &str) -> Option<Vec<String>> {
        let read = self.builds.read().await;
        if let Some((vals, time)) = read.get(key) {
            if Utc::now() - *time < chrono::Duration::minutes(5) {
                return Some(vals.clone());
            }
        }
        None
    }

    pub async fn set_builds(&self, key: String, vals: Vec<String>) {
        let mut write = self.builds.write().await;
        write.insert(key, (vals, Utc::now()));
    }
}
