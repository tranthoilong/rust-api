use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::banner_item::BannerItem;
use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateBannerItemInput {
    pub banner_id: Uuid,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub position: Option<i32>,
}

pub struct CreateBannerItemUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl CreateBannerItemUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateBannerItemInput) -> Result<BannerItem, String> {
        let item = BannerItem {
            id: Uuid::new_v4(),
            banner_id: input.banner_id,
            title: input.title,
            subtitle: input.subtitle,
            image_url: input.image_url,
            link_url: input.link_url,
            position: input.position,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        self.repo.create_item(item).await
    }
}
