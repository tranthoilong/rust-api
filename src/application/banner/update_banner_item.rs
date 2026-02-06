use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::banner_item::BannerItem;
use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateBannerItemInput {
    pub banner_id: Option<Uuid>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub position: Option<i32>,
}

pub struct UpdateBannerItemUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl UpdateBannerItemUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        input: UpdateBannerItemInput,
    ) -> Result<BannerItem, String> {
        let existing = self
            .repo
            .find_item_by_id(id)
            .await?
            .ok_or_else(|| "Banner item not found".to_string())?;

        let item = BannerItem {
            id: existing.id,
            banner_id: input.banner_id.unwrap_or(existing.banner_id),
            title: input.title.or(existing.title),
            subtitle: input.subtitle.or(existing.subtitle),
            image_url: input.image_url.or(existing.image_url),
            link_url: input.link_url.or(existing.link_url),
            position: input.position.or(existing.position),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update_item(item).await
    }
}
