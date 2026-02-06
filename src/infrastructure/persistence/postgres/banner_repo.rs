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

    async fn find_item_by_id(&self, id: Uuid) -> Result<Option<BannerItem>, String> {
        sqlx::query_as!(
            BannerItem,
            r#"
            SELECT id, banner_id, title, subtitle, image_url, link_url, position,
                   created_at, updated_at, deleted_at
            FROM banner_items
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, banner: Banner) -> Result<Banner, String> {
        let created = sqlx::query_as!(
            Banner,
            r#"
            INSERT INTO banners (
                id, title, slug, "key", description, is_active, starts_at, ends_at,
                created_at, updated_at, deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW(), NULL)
            RETURNING id, title, slug, "key", description, is_active, starts_at, ends_at,
                      created_at, updated_at, deleted_at
            "#,
            banner.id,
            banner.title,
            banner.slug,
            banner.key,
            banner.description,
            banner.is_active,
            banner.starts_at,
            banner.ends_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(created)
    }

    async fn update(&self, banner: Banner) -> Result<Banner, String> {
        let updated = sqlx::query_as!(
            Banner,
            r#"
            UPDATE banners
            SET title = $2,
                slug = $3,
                "key" = $4,
                description = $5,
                is_active = $6,
                starts_at = $7,
                ends_at = $8,
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, title, slug, "key", description, is_active, starts_at, ends_at,
                      created_at, updated_at, deleted_at
            "#,
            banner.id,
            banner.title,
            banner.slug,
            banner.key,
            banner.description,
            banner.is_active,
            banner.starts_at,
            banner.ends_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(updated)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE banners
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        if ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            UPDATE banners
            SET deleted_at = NOW()
            WHERE id = ANY($1) AND deleted_at IS NULL
            "#,
            ids
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_item(&self, item: BannerItem) -> Result<BannerItem, String> {
        let created = sqlx::query_as!(
            BannerItem,
            r#"
            INSERT INTO banner_items (
                id, banner_id, title, subtitle, image_url, link_url, position,
                created_at, updated_at, deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), NULL)
            RETURNING id, banner_id, title, subtitle, image_url, link_url, position,
                      created_at, updated_at, deleted_at
            "#,
            item.id,
            item.banner_id,
            item.title,
            item.subtitle,
            item.image_url,
            item.link_url,
            item.position,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(created)
    }

    async fn update_item(&self, item: BannerItem) -> Result<BannerItem, String> {
        let updated = sqlx::query_as!(
            BannerItem,
            r#"
            UPDATE banner_items
            SET banner_id = $2,
                title = $3,
                subtitle = $4,
                image_url = $5,
                link_url = $6,
                position = $7,
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, banner_id, title, subtitle, image_url, link_url, position,
                      created_at, updated_at, deleted_at
            "#,
            item.id,
            item.banner_id,
            item.title,
            item.subtitle,
            item.image_url,
            item.link_url,
            item.position,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(updated)
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE banner_items
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

