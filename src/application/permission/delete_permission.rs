use crate::domain::repositories::permission_repository::PermissionRepository;
use std::sync::Arc;

pub struct DeletePermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

use uuid::Uuid;

impl DeletePermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.delete(id).await
    }
}
