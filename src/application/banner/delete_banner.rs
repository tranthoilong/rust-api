use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::banner_repository::BannerRepository;

pub struct DeleteBannerUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl DeleteBannerUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.soft_delete(id).await
    }
}
