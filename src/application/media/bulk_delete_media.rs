use crate::domain::repositories::media_repository::MediaRepository;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BulkDeleteMediaInput {
    pub ids: Vec<Uuid>,
}

pub struct BulkDeleteMediaUseCase<R: MediaRepository> {
    repo: R,
}

impl<R: MediaRepository> BulkDeleteMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: BulkDeleteMediaInput) -> Result<(), String> {
        self.repo.soft_delete_many(&input.ids).await
    }
}
