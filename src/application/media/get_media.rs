use crate::domain::{entities::media::Media, repositories::media_repository::MediaRepository};
use crate::shared::utils::query::{ListParams, PaginatedResult};

pub struct GetMediaUseCase<R: MediaRepository> {
    repo: R,
}

use uuid::Uuid;

impl<R: MediaRepository> GetMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Media>, String> {
        self.repo.find_by_id(id).await
    }

    #[allow(dead_code)]
    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String> {
        self.repo.find_by_user_id(user_id).await
    }

    pub async fn get_by_user_paginated(
        &self,
        user_id: Uuid,
        params: &ListParams,
    ) -> Result<PaginatedResult<Media>, String> {
        self.repo.find_paginated(params, Some(user_id)).await
    }

    #[allow(dead_code)]
    pub async fn get_all_paginated(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<Media>, String> {
        self.repo.find_paginated(params, None).await
    }
}
