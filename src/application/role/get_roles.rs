use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct GetRolesUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl GetRolesUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Role>, String> {
        self.repo.find_all().await
    }
}
