use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::banner::Banner;
use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateBannerInput {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub key: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub ends_at: Option<chrono::NaiveDateTime>,
}

pub struct UpdateBannerUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl UpdateBannerUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, input: UpdateBannerInput) -> Result<Banner, String> {
        let existing = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| "Banner not found".to_string())?;

        let banner = Banner {
            id: existing.id,
            title: input.title.unwrap_or(existing.title),
            slug: input.slug.or(existing.slug),
            key: input.key.or(existing.key),
            description: input.description.or(existing.description),
            is_active: input.is_active.or(existing.is_active),
            starts_at: input.starts_at.or(existing.starts_at),
            ends_at: input.ends_at.or(existing.ends_at),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update(banner).await
    }
}
