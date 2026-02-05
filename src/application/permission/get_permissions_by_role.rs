use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct GetPermissionsByRoleUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl GetPermissionsByRoleUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, role_id: i32) -> Result<Vec<Permission>, String> {
        self.repo.find_by_role_id(role_id).await
    }
}
