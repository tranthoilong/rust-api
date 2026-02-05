use std::sync::Arc;

use uuid::Uuid;

use crate::domain::repositories::category_repository::CategoryRepository;

pub struct DeleteCategoryUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl DeleteCategoryUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.soft_delete(id).await
    }
}

