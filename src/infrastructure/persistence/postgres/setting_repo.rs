use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::setting::Setting;
use crate::domain::repositories::setting_repository::SettingRepository;

pub struct PgSettingRepository {
    pool: Pool<Postgres>,
}

impl PgSettingRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingRepository for PgSettingRepository {
    async fn get_by_key(&self, key: &str) -> Result<Option<Setting>, String> {
        sqlx::query_as!(
            Setting,
            r#"
            SELECT id, key, value as "value: _", description, created_at, updated_at, deleted_at
            FROM settings
            WHERE key = $1 AND deleted_at IS NULL
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn set(&self, key: &str, value: Option<serde_json::Value>) -> Result<Setting, String> {
        sqlx::query_as!(
            Setting,
            r#"
            INSERT INTO settings (key, value)
            VALUES ($1, $2)
            ON CONFLICT (key)
            DO UPDATE SET value = EXCLUDED.value, updated_at = NOW(), deleted_at = NULL
            RETURNING id, key, value as "value: _", description, created_at, updated_at, deleted_at
            "#,
            key,
            value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list(&self) -> Result<Vec<Setting>, String> {
        sqlx::query_as!(
            Setting,
            r#"
            SELECT id, key, value as "value: _", description, created_at, updated_at, deleted_at
            FROM settings
            WHERE deleted_at IS NULL
            ORDER BY key
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

