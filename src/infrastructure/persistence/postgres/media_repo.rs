use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::media::{Media, NewMedia};
use crate::domain::repositories::media_repository::MediaRepository;

pub struct PgMediaRepository {
    pool: Pool<Postgres>,
}

impl PgMediaRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaRepository for PgMediaRepository {
    async fn create(&self, media: NewMedia) -> Result<Media, String> {
        sqlx::query_as!(
            Media,
            r#"
            INSERT INTO media (user_id, media_type, file_path)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, media_type, file_path, created_at, updated_at, deleted_at
            "#,
            media.user_id,
            media.media_type,
            media.file_path
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Media>, String> {
        sqlx::query_as!(
            Media,
            r#"SELECT id, user_id, media_type, file_path, created_at, updated_at, deleted_at FROM media WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Media>, String> {
        sqlx::query_as!(
            Media,
            r#"SELECT id, user_id, media_type, file_path, created_at, updated_at, deleted_at FROM media WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
