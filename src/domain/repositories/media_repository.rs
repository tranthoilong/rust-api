use crate::domain::entities::media::{Media, NewMedia};
use crate::shared::utils::query::{ListParams, PaginatedResult};
use async_trait::async_trait;

use uuid::Uuid;

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn create(&self, media: NewMedia) -> Result<Media, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Media>, String>;
    async fn find_paginated(
        &self,
        params: &ListParams,
        user_filter: Option<Uuid>,
    ) -> Result<PaginatedResult<Media>, String>;
    #[allow(dead_code)]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String>;
}

#[async_trait]
impl<T: MediaRepository + ?Sized + Send + Sync> MediaRepository for std::sync::Arc<T> {
    async fn create(&self, media: NewMedia) -> Result<Media, String> {
        (**self).create(media).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Media>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_paginated(
        &self,
        params: &ListParams,
        user_filter: Option<Uuid>,
    ) -> Result<PaginatedResult<Media>, String> {
        (**self).find_paginated(params, user_filter).await
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String> {
        (**self).find_by_user_id(user_id).await
    }
}
