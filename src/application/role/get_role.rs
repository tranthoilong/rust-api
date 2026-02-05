use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct GetRoleUseCase {
    repo: Arc<dyn RoleRepository>,
}

use uuid::Uuid;

impl GetRoleUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<Role>, String> {
        self.repo.find_by_id(id).await
    }
}
