use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::banner::Banner;
use crate::domain::entities::banner_item::BannerItem;
use crate::domain::repositories::banner_repository::BannerRepository;

pub struct PgBannerRepository {
    pool: Pool<Postgres>,
}

impl PgBannerRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BannerRepository for PgBannerRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Banner>, String> {
        sqlx::query_as!(
            Banner,
            r#"
            SELECT id, title, slug, "key", description, is_active, starts_at, ends_at,
                   created_at, updated_at, deleted_at
            FROM banners
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_by_key(&self, key: &str) -> Result<Option<Banner>, String> {
        sqlx::query_as!(
            Banner,
            r#"
            SELECT id, title, slug, "key", description, is_active, starts_at, ends_at,
                   created_at, updated_at, deleted_at
            FROM banners
            WHERE "key" = $1 AND deleted_at IS NULL
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_active(&self) -> Result<Vec<Banner>, String> {
        sqlx::query_as!(
            Banner,
            r#"
            SELECT id, title, slug, "key", description, is_active, starts_at, ends_at,
                   created_at, updated_at, deleted_at
            FROM banners
            WHERE deleted_at IS NULL
              AND is_active = TRUE
              AND (starts_at IS NULL OR starts_at <= NOW())
              AND (ends_at IS NULL OR ends_at >= NOW())
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_items(&self, banner_id: Uuid) -> Result<Vec<BannerItem>, String> {
        sqlx::query_as!(
            BannerItem,
            r#"
            SELECT id, banner_id, title, subtitle, image_url, link_url, position,
                   created_at, updated_at, deleted_at
            FROM banner_items
            WHERE banner_id = $1 AND deleted_at IS NULL
            ORDER BY position NULLS LAST, created_at ASC
            "#,
            banner_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

