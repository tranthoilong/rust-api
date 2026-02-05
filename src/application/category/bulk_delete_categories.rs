use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::category_repository::CategoryRepository;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BulkDeleteCategoriesInput {
    pub ids: Vec<Uuid>,
}

pub struct BulkDeleteCategoriesUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl BulkDeleteCategoriesUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: BulkDeleteCategoriesInput) -> Result<(), String> {
        self.repo.soft_delete_many(&input.ids).await
    }
}

