use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct DeleteRoleUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl DeleteRoleUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i32) -> Result<(), String> {
        self.repo.delete(id).await
    }
}
