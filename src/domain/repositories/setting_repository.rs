use crate::domain::entities::setting::Setting;
use async_trait::async_trait;

#[async_trait]
pub trait SettingRepository: Send + Sync {
    async fn get_by_key(&self, key: &str) -> Result<Option<Setting>, String>;
    async fn set(&self, key: &str, value: Option<serde_json::Value>) -> Result<Setting, String>;
    async fn list(&self) -> Result<Vec<Setting>, String>;
}

#[async_trait]
impl<T: SettingRepository + ?Sized + Send + Sync> SettingRepository for std::sync::Arc<T> {
    async fn get_by_key(&self, key: &str) -> Result<Option<Setting>, String> {
        (**self).get_by_key(key).await
    }

    async fn set(&self, key: &str, value: Option<serde_json::Value>) -> Result<Setting, String> {
        (**self).set(key, value).await
    }

    async fn list(&self) -> Result<Vec<Setting>, String> {
        (**self).list().await
    }
}

