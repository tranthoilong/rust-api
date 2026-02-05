use std::sync::Arc;

use crate::domain::entities::setting::Setting;
use crate::domain::repositories::setting_repository::SettingRepository;

pub struct UpdateSettingUseCase {
    repo: Arc<dyn SettingRepository>,
}

impl UpdateSettingUseCase {
    pub fn new(repo: Arc<dyn SettingRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        key: &str,
        value: Option<serde_json::Value>,
    ) -> Result<Setting, String> {
        self.repo.set(key, value).await
    }
}

