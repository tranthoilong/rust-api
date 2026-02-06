use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::tag_repository::TagRepository;

pub struct DeleteTagUseCase {
    repo: Arc<dyn TagRepository>,
}

impl DeleteTagUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.soft_delete(id).await
    }
}
