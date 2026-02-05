use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct GetPermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

use uuid::Uuid;

impl GetPermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<Permission>, String> {
        self.repo.find_by_id(id).await
    }
}
