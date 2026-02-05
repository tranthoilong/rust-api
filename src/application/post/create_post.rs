use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::post::{Post, PostStatus};
use crate::domain::repositories::post_repository::PostRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreatePostInput {
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub status: Option<PostStatus>,
    pub published_at: Option<chrono::NaiveDateTime>,
}

pub struct CreatePostUseCase {
    repo: Arc<dyn PostRepository>,
}

impl CreatePostUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreatePostInput) -> Result<Post, String> {
        let post = Post {
            id: Uuid::new_v4(),
            author_id: input.author_id,
            category_id: input.category_id,
            title: input.title,
            slug: input.slug,
            excerpt: input.excerpt,
            content: input.content,
            status: input.status.or(Some(PostStatus::Draft)),
            published_at: input.published_at,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        self.repo.create(post).await
    }
}

