use std::sync::Arc;

use crate::domain::entities::language::Language;
use crate::domain::repositories::language_repository::LanguageRepository;

pub struct GetLanguagesUseCase {
    repo: Arc<dyn LanguageRepository + Send + Sync>,
}

impl GetLanguagesUseCase {
    pub fn new(repo: Arc<dyn LanguageRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Language>, String> {
        self.repo
            .list_all()
            .await
            .map_err(|e| format!("Failed to list languages: {}", e))
    }
}


