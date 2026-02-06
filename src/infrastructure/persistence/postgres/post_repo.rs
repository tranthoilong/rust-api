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

    async fn create(&self, post: Post) -> Result<Post, String> {
        let created = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (
                id, author_id, category_id, title, slug, excerpt, content,
                published_at, created_at, updated_at, deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW(), NULL)
            RETURNING id, author_id, category_id, title, slug, excerpt, content,
                      status as "status: PostStatus", published_at,
                      created_at, updated_at, deleted_at
            "#,
            post.id,
            post.author_id,
            post.category_id,
            post.title,
            post.slug,
            post.excerpt,
            post.content,
            post.published_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(created)
    }

    async fn update(&self, post: Post) -> Result<Post, String> {
        let updated = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET author_id = $2,
                category_id = $3,
                title = $4,
                slug = $5,
                excerpt = $6,
                content = $7,
                published_at = $8,
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, author_id, category_id, title, slug, excerpt, content,
                      status as "status: PostStatus", published_at,
                      created_at, updated_at, deleted_at
            "#,
            post.id,
            post.author_id,
            post.category_id,
            post.title,
            post.slug,
            post.excerpt,
            post.content,
            post.published_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(updated)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE posts
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
            UPDATE posts
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
}
