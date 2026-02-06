use std::sync::Arc;

use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

pub struct GetTagsUseCase {
    repo: Arc<dyn TagRepository>,
}

impl GetTagsUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, r#type: &str) -> Result<Vec<Tag>, String> {
        self.repo.list_by_type(r#type).await
    }
}
