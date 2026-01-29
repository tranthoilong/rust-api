use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct GetPermissionsUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl GetPermissionsUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Permission>, String> {
        self.repo.find_all().await
    }
}
