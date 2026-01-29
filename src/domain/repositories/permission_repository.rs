use crate::domain::entities::permission::{NewPermission, Permission, UpdatePermission};
use async_trait::async_trait;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Permission>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Permission>, String>;
    async fn create(&self, permission: NewPermission) -> Result<Permission, String>;
    async fn update(&self, id: i32, permission: UpdatePermission) -> Result<Permission, String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
}

#[async_trait]
impl<T: PermissionRepository + ?Sized + Send + Sync> PermissionRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<Permission>, String> {
        (**self).find_all().await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Permission>, String> {
        (**self).find_by_id(id).await
    }

    async fn create(&self, permission: NewPermission) -> Result<Permission, String> {
        (**self).create(permission).await
    }

    async fn update(&self, id: i32, permission: UpdatePermission) -> Result<Permission, String> {
        (**self).update(id, permission).await
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        (**self).delete(id).await
    }
}
