use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::post::{Post, PostStatus};
use crate::domain::repositories::post_repository::{PostRepository, PostSearchFilter};

pub struct PgPostRepository {
    pool: Pool<Postgres>,
}

impl PgPostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PgPostRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, String> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, category_id, title, slug, excerpt, content,
                   status as "status: PostStatus", published_at,
                   created_at, updated_at, deleted_at
            FROM posts
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, String> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, category_id, title, slug, excerpt, content,
                   status as "status: PostStatus", published_at,
                   created_at, updated_at, deleted_at
            FROM posts
            WHERE slug = $1 AND deleted_at IS NULL
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn search(&self, _filter: &PostSearchFilter) -> Result<Vec<Post>, String> {
        // Để đảm bảo không lỗi runtime, tạm thời chỉ trả về các bài viết đã publish,
        // có thể mở rộng sau để filter theo category/status/search bằng QueryBuilder.
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, category_id, title, slug, excerpt, content,
                   status as "status: PostStatus", published_at,
                   created_at, updated_at, deleted_at
            FROM posts
            WHERE deleted_at IS NULL
              AND status = 'published'
            ORDER BY published_at DESC NULLS LAST, created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(posts)
    }
}

