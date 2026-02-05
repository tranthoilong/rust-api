use crate::domain::entities::media::{Media, NewMedia};
use crate::shared::utils::query::PaginatedResult;
use async_trait::async_trait;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MediaSearchFilter {
    pub search: Option<String>,
    pub user_id: Option<Uuid>,
}

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn create(&self, media: NewMedia) -> Result<Media, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Media>, String>;
    /// Cập nhật metadata cho 1 media (theo id trong struct)
    async fn update(&self, media: Media) -> Result<Media, String>;
    async fn search(
        &self,
        filter: &MediaSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Media>, String>;
    #[allow(dead_code)]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String>;

    /// Xoá mềm 1 media
    async fn soft_delete(&self, id: Uuid) -> Result<(), String>;
    /// Xoá mềm nhiều media
    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String>;
}

#[async_trait]
impl<T: MediaRepository + ?Sized + Send + Sync> MediaRepository for std::sync::Arc<T> {
    async fn create(&self, media: NewMedia) -> Result<Media, String> {
        (**self).create(media).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Media>, String> {
        (**self).find_by_id(id).await
    }

    async fn update(&self, media: Media) -> Result<Media, String> {
        (**self).update(media).await
    }

    async fn search(
        &self,
        filter: &MediaSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Media>, String> {
        (**self).search(filter, sort_by, cursor, limit).await
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String> {
        (**self).find_by_user_id(user_id).await
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        (**self).soft_delete(id).await
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        (**self).soft_delete_many(ids).await
    }
}
