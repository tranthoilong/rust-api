use crate::domain::entities::permission::{NewPermission, Permission, UpdatePermission};
use crate::shared::utils::query::{ListParams, PaginatedResult};
use async_trait::async_trait;

use uuid::Uuid;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_all(&self) -> Result<Vec<Permission>, String>;
    async fn find_paginated(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<Permission>, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, String>;
    async fn create(&self, permission: NewPermission) -> Result<Permission, String>;
    async fn update(&self, id: Uuid, permission: UpdatePermission) -> Result<Permission, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
    async fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<Permission>, String>;
    async fn assign_to_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String>;
    async fn revoke_from_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String>;
}

#[async_trait]
impl<T: PermissionRepository + ?Sized + Send + Sync> PermissionRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<Permission>, String> {
        (**self).find_all().await
    }

    async fn find_paginated(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<Permission>, String> {
        (**self).find_paginated(params).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, String> {
        (**self).find_by_id(id).await
    }

    async fn create(&self, permission: NewPermission) -> Result<Permission, String> {
        (**self).create(permission).await
    }

    async fn update(&self, id: Uuid, permission: UpdatePermission) -> Result<Permission, String> {
        (**self).update(id, permission).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        (**self).delete(id).await
    }

    async fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<Permission>, String> {
        (**self).find_by_role_id(role_id).await
    }

    async fn assign_to_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        (**self).assign_to_role(role_id, permission_id).await
    }

    async fn revoke_from_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        (**self).revoke_from_role(role_id, permission_id).await
    }
}
