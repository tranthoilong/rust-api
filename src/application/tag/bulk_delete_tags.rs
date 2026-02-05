use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::tag_repository::TagRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BulkDeleteTagsInput {
    pub ids: Vec<Uuid>,
}

pub struct BulkDeleteTagsUseCase {
    repo: Arc<dyn TagRepository>,
}

impl BulkDeleteTagsUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: BulkDeleteTagsInput) -> Result<(), String> {
        self.repo.soft_delete_many(&input.ids).await
    }
}

