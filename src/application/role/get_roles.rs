use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::shared::utils::query::{ListParams, PaginatedResult};
use std::sync::Arc;

pub struct GetRolesUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl GetRolesUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: &ListParams) -> Result<PaginatedResult<Role>, String> {
        self.repo.find_paginated(params).await
    }
}
