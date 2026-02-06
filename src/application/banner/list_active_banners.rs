use std::sync::Arc;

use crate::domain::entities::banner::Banner;
use crate::domain::repositories::banner_repository::BannerRepository;

pub struct ListActiveBannersUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl ListActiveBannersUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Banner>, String> {
        self.repo.list_active().await
    }
}
