use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::post_repository::PostRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BulkDeletePostsInput {
    pub ids: Vec<Uuid>,
}

pub struct BulkDeletePostsUseCase {
    repo: Arc<dyn PostRepository>,
}

impl BulkDeletePostsUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: BulkDeletePostsInput) -> Result<(), String> {
        self.repo.soft_delete_many(&input.ids).await
    }
}

