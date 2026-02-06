use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateCategoryInput {
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
}

pub struct CreateCategoryUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl CreateCategoryUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateCategoryInput) -> Result<Category, String> {
        let category = Category {
            id: Uuid::new_v4(),
            parent_id: input.parent_id,
            name: input.name,
            slug: input.slug,
            r#type: input.r#type,
            description: input.description,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        self.repo.create(category).await
    }
}
