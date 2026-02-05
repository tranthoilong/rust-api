use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::entities::banner::Banner;
use crate::domain::entities::banner_item::BannerItem;
use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerWithItems {
    pub banner: Banner,
    pub items: Vec<BannerItem>,
}

pub struct GetBannerByKeyUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl GetBannerByKeyUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, key: &str) -> Result<Option<BannerWithItems>, String> {
        if let Some(banner) = self.repo.get_by_key(key).await? {
            let items = self.repo.list_items(banner.id).await?;
            Ok(Some(BannerWithItems { banner, items }))
        } else {
            Ok(None)
        }
    }
}

