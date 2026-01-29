use crate::domain::entities::role::{Role, UpdateRole};
use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct UpdateRoleUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl UpdateRoleUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i32, role: UpdateRole) -> Result<Role, String> {
        self.repo.update(id, role).await
    }
}
