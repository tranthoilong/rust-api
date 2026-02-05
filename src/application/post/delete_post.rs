use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::post_repository::PostRepository;

pub struct DeletePostUseCase {
    repo: Arc<dyn PostRepository>,
}

impl DeletePostUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.soft_delete(id).await
    }
}

