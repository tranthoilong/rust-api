use std::sync::Arc;

use crate::domain::entities::language::Language;
use crate::domain::repositories::language_repository::LanguageRepository;

pub struct GetDefaultLanguageUseCase {
    repo: Arc<dyn LanguageRepository + Send + Sync>,
}

impl GetDefaultLanguageUseCase {
    pub fn new(repo: Arc<dyn LanguageRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Option<Language>, String> {
        self.repo
            .get_default()
            .await
            .map_err(|e| format!("Failed to get default language: {}", e))
    }
}
