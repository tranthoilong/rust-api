use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct AssignRoleToUserUseCase {
    repo: Arc<dyn RoleRepository>,
}

impl AssignRoleToUserUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: i32, role_id: i32) -> Result<(), String> {
        self.repo.assign_to_user(user_id, role_id).await
    }
}
