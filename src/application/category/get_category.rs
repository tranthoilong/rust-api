use std::sync::Arc;

use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

pub struct GetCategoryUseCase {
    repo: Arc<dyn CategoryRepository>,
}

impl GetCategoryUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, slug: &str, r#type: &str) -> Result<Option<Category>, String> {
        self.repo.find_by_slug_and_type(slug, r#type).await
    }
}
