use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateTagInput {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateTagUseCase {
    repo: Arc<dyn TagRepository>,
}

impl UpdateTagUseCase {
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, input: UpdateTagInput) -> Result<Tag, String> {
        let existing = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Tag not found".to_string())?;

        let tag = Tag {
            id: existing.id,
            name: input.name.unwrap_or(existing.name),
            slug: input.slug.or(existing.slug),
            r#type: input.r#type.unwrap_or(existing.r#type),
            description: input.description.or(existing.description),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update(tag).await
    }
}

