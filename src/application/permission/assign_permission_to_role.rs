use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct AssignPermissionToRoleUseCase {
    repo: Arc<dyn PermissionRepository>,
}

use uuid::Uuid;

impl AssignPermissionToRoleUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        self.repo.assign_to_role(role_id, permission_id).await
    }
}
