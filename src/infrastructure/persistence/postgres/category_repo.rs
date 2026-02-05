use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

pub struct PgCategoryRepository {
    pool: Pool<Postgres>,
}

impl PgCategoryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PgCategoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, String> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, parent_id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM categories
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
    ) -> Result<Option<Category>, String> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, parent_id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM categories
            WHERE slug = $1 AND type = $2 AND deleted_at IS NULL
            "#,
            slug,
            r#type
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Category>, String> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, parent_id, name, slug, type, description,
                   created_at, updated_at, deleted_at
            FROM categories
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

