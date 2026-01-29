use crate::domain::entities::role::{NewRole, Role};
use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct CreateRoleUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl CreateRoleUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, role: NewRole) -> Result<Role, String> {
        self.repo.create(role).await
    }
}
