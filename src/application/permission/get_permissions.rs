use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::shared::utils::query::{ListParams, PaginatedResult};
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
        self.repo.find_paginated(params).await
    }
}
