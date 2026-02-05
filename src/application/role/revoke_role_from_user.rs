use crate::domain::repositories::role_repository::RoleRepository;
use std::sync::Arc;

pub struct RevokeRoleFromUserUseCase {
    repo: Arc<dyn RoleRepository>,
}

use uuid::Uuid;

impl RevokeRoleFromUserUseCase {
    pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String> {
        self.repo.revoke_from_user(user_id, role_id).await
    }
}
