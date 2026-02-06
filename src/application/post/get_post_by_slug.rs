use std::sync::Arc;

use crate::domain::entities::post::Post;
use crate::domain::repositories::post_repository::PostRepository;

pub struct GetPostBySlugUseCase {
    repo: Arc<dyn PostRepository>,
}

impl GetPostBySlugUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, slug: &str) -> Result<Option<Post>, String> {
        self.repo.find_by_slug(slug).await
    }
}
