use crate::domain::entities::permission::{Permission, UpdatePermission};
use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct UpdatePermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

use uuid::Uuid;

impl UpdatePermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        permission: UpdatePermission,
    ) -> Result<Permission, String> {
        self.repo.update(id, permission).await
    }
}
