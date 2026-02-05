use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

pub struct PgTagRepository {
    pool: Pool<Postgres>,
}

impl PgTagRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PgTagRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, String> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM tags
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_slug_and_type(
        &self,
        slug: &str,
        r#type: &str,
    ) -> Result<Option<Tag>, String> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM tags
            WHERE slug = $1 AND type = $2 AND deleted_at IS NULL
            "#,
            slug,
            r#type
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Tag>, String> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM tags
            WHERE type = $1 AND deleted_at IS NULL
            ORDER BY name
            "#,
            r#type
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

