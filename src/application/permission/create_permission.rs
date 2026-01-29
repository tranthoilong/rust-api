use crate::domain::entities::permission::{NewPermission, Permission};
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct CreatePermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl CreatePermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, permission: NewPermission) -> Result<Permission, String> {
        self.repo.create(permission).await
    }
}
