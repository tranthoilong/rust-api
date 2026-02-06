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
    async fn create(&self, category: Category) -> Result<Category, String>;
    async fn update(&self, category: Category) -> Result<Category, String>;
    async fn soft_delete(&self, id: Uuid) -> Result<(), String>;
    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String>;
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

    async fn create(&self, category: Category) -> Result<Category, String> {
        (**self).create(category).await
    }

    async fn update(&self, category: Category) -> Result<Category, String> {
        (**self).update(category).await
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        (**self).soft_delete(id).await
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        (**self).soft_delete_many(ids).await
    }
}
