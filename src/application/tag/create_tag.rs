use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateTagInput {
    pub name: String,
    pub slug: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
}

pub struct CreateTagUseCase {
    repo: Arc<dyn TagRepository>,
}

impl CreateTagUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateTagInput) -> Result<Tag, String> {
        let tag = Tag {
            id: Uuid::new_v4(),
            name: input.name,
            slug: input.slug,
            r#type: input.r#type,
            description: input.description,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        self.repo.create(tag).await
    }
}

