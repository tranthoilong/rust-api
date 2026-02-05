use crate::domain::entities::category::Category;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, String>;
    async fn find_by_slug_and_type(
        &self,
        slug: &str,
        r#type: &str,
    ) -> Result<Option<Category>, String>;
    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Category>, String>;
}

#[async_trait]
impl<T: CategoryRepository + ?Sized + Send + Sync> CategoryRepository for std::sync::Arc<T> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_by_slug_and_type(
        &self,
        slug: &str,
        r#type: &str,
    ) -> Result<Option<Category>, String> {
        (**self).find_by_slug_and_type(slug, r#type).await
    }

    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Category>, String> {
        (**self).list_by_type(r#type).await
    }
}

