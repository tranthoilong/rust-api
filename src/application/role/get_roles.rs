use crate::application::common::list_params::{ListParams, PaginatedResult};
use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::{RoleRepository, RoleSearchFilter};
use std::sync::Arc;

pub struct GetRolesUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl GetRolesUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: &ListParams) -> Result<PaginatedResult<Role>, String> {
        let filter = RoleSearchFilter {
            search: params.search.clone(),
        };
        let limit = params.limit.unwrap_or(20).clamp(1, 100);
        self.repo
            .search(&filter, params.sort_by.clone(), params.cursor.clone(), limit)
            .await
    }
}
