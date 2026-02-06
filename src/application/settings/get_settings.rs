use std::sync::Arc;

use crate::domain::entities::setting::Setting;
use crate::domain::repositories::setting_repository::SettingRepository;

pub struct GetSettingsUseCase {
    repo: Arc<dyn SettingRepository>,
}

impl GetSettingsUseCase {
    pub fn new(repo: Arc<dyn SettingRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Setting>, String> {
        self.repo.list().await
    }
}
