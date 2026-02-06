use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::post::{Post, PostStatus};
use crate::domain::repositories::post_repository::PostRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdatePostInput {
    pub category_id: Option<Uuid>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub status: Option<PostStatus>,
    pub published_at: Option<chrono::NaiveDateTime>,
}

pub struct UpdatePostUseCase {
    repo: Arc<dyn PostRepository>,
}

impl UpdatePostUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, input: UpdatePostInput) -> Result<Post, String> {
        let existing = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Post not found".to_string())?;

        let post = Post {
            id: existing.id,
            author_id: existing.author_id,
            category_id: input.category_id.or(existing.category_id),
            title: input.title.unwrap_or(existing.title),
            slug: input.slug.or(existing.slug),
            excerpt: input.excerpt.or(existing.excerpt),
            content: input.content.or(existing.content),
            status: input.status.or(existing.status),
            published_at: input.published_at.or(existing.published_at),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update(post).await
    }
}

