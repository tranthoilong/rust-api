use std::sync::Arc;

use crate::domain::entities::setting::Setting;
use crate::domain::repositories::setting_repository::SettingRepository;

pub struct GetSettingUseCase {
    repo: Arc<dyn SettingRepository>,
}

impl GetSettingUseCase {
    pub fn new(repo: Arc<dyn SettingRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, key: &str) -> Result<Option<Setting>, String> {
        self.repo.get_by_key(key).await
    }
}

