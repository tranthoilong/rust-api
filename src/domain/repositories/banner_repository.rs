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
    async fn create(&self, banner: Banner) -> Result<Banner, String>;
    async fn update(&self, banner: Banner) -> Result<Banner, String>;
    async fn soft_delete(&self, id: Uuid) -> Result<(), String>;
    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String>;

    async fn list_items(&self, banner_id: Uuid) -> Result<Vec<BannerItem>, String>;
    async fn find_item_by_id(&self, id: Uuid) -> Result<Option<BannerItem>, String>;
    async fn create_item(&self, item: BannerItem) -> Result<BannerItem, String>;
    async fn update_item(&self, item: BannerItem) -> Result<BannerItem, String>;
    async fn delete_item(&self, id: Uuid) -> Result<(), String>;
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

    async fn find_item_by_id(&self, id: Uuid) -> Result<Option<BannerItem>, String> {
        (**self).find_item_by_id(id).await
    }

    async fn create(&self, banner: Banner) -> Result<Banner, String> {
        (**self).create(banner).await
    }

    async fn update(&self, banner: Banner) -> Result<Banner, String> {
        (**self).update(banner).await
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        (**self).soft_delete(id).await
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        (**self).soft_delete_many(ids).await
    }

    async fn create_item(&self, item: BannerItem) -> Result<BannerItem, String> {
        (**self).create_item(item).await
    }

    async fn update_item(&self, item: BannerItem) -> Result<BannerItem, String> {
        (**self).update_item(item).await
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), String> {
        (**self).delete_item(id).await
    }
}
