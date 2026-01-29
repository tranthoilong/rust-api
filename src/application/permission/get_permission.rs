use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct GetPermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl GetPermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i32) -> Result<Option<Permission>, String> {
        self.repo.find_by_id(id).await
    }
}
