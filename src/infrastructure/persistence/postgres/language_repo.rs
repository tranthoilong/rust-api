use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::language::Language;
use crate::domain::repositories::language_repository::LanguageRepository;

#[allow(dead_code)]
pub struct PgLanguageRepository {
    pool: Pool<Postgres>,
}

#[allow(dead_code)]
impl PgLanguageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LanguageRepository for PgLanguageRepository {
    async fn list_all(&self) -> Result<Vec<Language>, String> {
        sqlx::query_as!(
            Language,
            r#"
            SELECT id, code, name, is_default, created_at, updated_at, deleted_at
            FROM languages
            WHERE deleted_at IS NULL
            ORDER BY code
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_default(&self) -> Result<Option<Language>, String> {
        sqlx::query_as!(
            Language,
            r#"
            SELECT id, code, name, is_default, created_at, updated_at, deleted_at
            FROM languages
            WHERE is_default = TRUE AND deleted_at IS NULL
            ORDER BY created_at
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Language>, String> {
        sqlx::query_as!(
            Language,
            r#"
            SELECT id, code, name, is_default, created_at, updated_at, deleted_at
            FROM languages
            WHERE code = $1 AND deleted_at IS NULL
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

