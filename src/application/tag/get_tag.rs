use std::sync::Arc;

use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

pub struct GetTagUseCase {
    repo: Arc<dyn TagRepository>,
}

impl GetTagUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, slug: &str, r#type: &str) -> Result<Option<Tag>, String> {
        self.repo.find_by_slug_and_type(slug, r#type).await
    }
}
