use crate::domain::entities::tag::Tag;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TagRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, String>;
    async fn find_by_slug_and_type(&self, slug: &str, r#type: &str) -> Result<Option<Tag>, String>;
    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Tag>, String>;
}

#[async_trait]
impl<T: TagRepository + ?Sized + Send + Sync> TagRepository for std::sync::Arc<T> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_by_slug_and_type(&self, slug: &str, r#type: &str) -> Result<Option<Tag>, String> {
        (**self).find_by_slug_and_type(slug, r#type).await
    }

    async fn list_by_type(&self, r#type: &str) -> Result<Vec<Tag>, String> {
        (**self).list_by_type(r#type).await
    }
}

