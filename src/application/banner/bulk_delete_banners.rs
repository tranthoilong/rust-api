use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BulkDeleteBannersInput {
    pub ids: Vec<Uuid>,
}

pub struct BulkDeleteBannersUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl BulkDeleteBannersUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: BulkDeleteBannersInput) -> Result<(), String> {
        self.repo.soft_delete_many(&input.ids).await
    }
}
