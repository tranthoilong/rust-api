use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::banner_repository::BannerRepository;

pub struct DeleteBannerItemUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl DeleteBannerItemUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.delete_item(id).await
    }
}
