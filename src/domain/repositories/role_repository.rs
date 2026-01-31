use crate::domain::entities::role::{NewRole, Role, UpdateRole};
use async_trait::async_trait;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Role>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, String>;
    async fn create(&self, role: NewRole) -> Result<Role, String>;
    async fn update(&self, id: i32, role: UpdateRole) -> Result<Role, String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, String>;
}

#[async_trait]
impl<T: RoleRepository + ?Sized + Send + Sync> RoleRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<Role>, String> {
        (**self).find_all().await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, String> {
        (**self).find_by_id(id).await
    }

    async fn create(&self, role: NewRole) -> Result<Role, String> {
        (**self).create(role).await
    }

    async fn update(&self, id: i32, role: UpdateRole) -> Result<Role, String> {
        (**self).update(id, role).await
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        (**self).delete(id).await
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, String> {
        (**self).find_by_user_id(user_id).await
    }
}
