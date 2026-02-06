use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::banner::Banner;
use crate::domain::repositories::banner_repository::BannerRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateBannerInput {
    pub title: String,
    pub slug: Option<String>,
    pub key: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub ends_at: Option<chrono::NaiveDateTime>,
}

pub struct CreateBannerUseCase {
    repo: Arc<dyn BannerRepository>,
}

impl CreateBannerUseCase {
    pub fn new(repo: Arc<dyn BannerRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateBannerInput) -> Result<Banner, String> {
        let banner = Banner {
            id: Uuid::new_v4(),
            title: input.title,
            slug: input.slug,
            key: input.key,
            description: input.description,
            is_active: input.is_active.or(Some(true)),
            starts_at: input.starts_at,
            ends_at: input.ends_at,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        self.repo.create(banner).await
    }
}
