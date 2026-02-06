use std::sync::Arc;

use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

pub struct GetCategoriesUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl GetCategoriesUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, r#type: &str) -> Result<Vec<Category>, String> {
        self.repo.list_by_type(r#type).await
    }
}
