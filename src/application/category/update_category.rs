use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateCategoryInput {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateCategoryUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl UpdateCategoryUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, input: UpdateCategoryInput) -> Result<Category, String> {
        let existing = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Category not found".to_string())?;

        let category = Category {
            id: existing.id,
            parent_id: input.parent_id.or(existing.parent_id),
            name: input.name.unwrap_or(existing.name),
            slug: input.slug.or(existing.slug),
            r#type: input.r#type.unwrap_or(existing.r#type),
            description: input.description.or(existing.description),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update(category).await
    }
}

