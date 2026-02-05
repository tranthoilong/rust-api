use crate::domain::entities::banner::Banner;
use crate::domain::entities::banner_item::BannerItem;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BannerRepository: Send + Sync {
    #[allow(dead_code)]
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Banner>, String>;
    async fn get_by_key(&self, key: &str) -> Result<Option<Banner>, String>;
    async fn list_active(&self) -> Result<Vec<Banner>, String>;

    async fn list_items(&self, banner_id: Uuid) -> Result<Vec<BannerItem>, String>;
}

#[async_trait]
impl<T: BannerRepository + ?Sized + Send + Sync> BannerRepository for std::sync::Arc<T> {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Banner>, String> {
        (**self).get_by_id(id).await
    }

    async fn get_by_key(&self, key: &str) -> Result<Option<Banner>, String> {
        (**self).get_by_key(key).await
    }

    async fn list_active(&self) -> Result<Vec<Banner>, String> {
        (**self).list_active().await
    }

    async fn list_items(&self, banner_id: Uuid) -> Result<Vec<BannerItem>, String> {
        (**self).list_items(banner_id).await
    }
}

