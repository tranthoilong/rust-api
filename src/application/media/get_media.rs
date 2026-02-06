use crate::application::common::list_params::{ListParams, PaginatedResult};
use crate::domain::{
    entities::media::Media,
    repositories::media_repository::{MediaRepository, MediaSearchFilter},
};

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
        let filter = MediaSearchFilter {
            search: params.search.clone(),
            user_id: Some(user_id),
        };
        let limit = params.limit.unwrap_or(20).clamp(1, 100);
        self.repo
            .search(
                &filter,
                params.sort_by.clone(),
                params.cursor.clone(),
                limit,
            )
            .await
    }

    #[allow(dead_code)]
    pub async fn get_all_paginated(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<Media>, String> {
        let filter = MediaSearchFilter {
            search: params.search.clone(),
            user_id: None,
        };
        let limit = params.limit.unwrap_or(20).clamp(1, 100);
        self.repo
            .search(
                &filter,
                params.sort_by.clone(),
                params.cursor.clone(),
                limit,
            )
            .await
    }
}
