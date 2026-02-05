use crate::application::common::list_params::{ListParams, PaginatedResult};
use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::{
    PermissionRepository, PermissionSearchFilter,
};
use std::sync::Arc;

pub struct GetPermissionsUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl GetPermissionsUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<Permission>, String> {
        let filter = PermissionSearchFilter {
            search: params.search.clone(),
        };
        let limit = params.limit.unwrap_or(20).clamp(1, 100);
        self.repo
            .search(&filter, params.sort_by.clone(), params.cursor.clone(), limit)
            .await
    }
}
