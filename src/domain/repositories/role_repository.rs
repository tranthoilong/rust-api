use crate::domain::entities::role::{NewRole, Role, UpdateRole};
use crate::shared::utils::query::PaginatedResult;
use async_trait::async_trait;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RoleSearchFilter {
    pub search: Option<String>,
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_all(&self) -> Result<Vec<Role>, String>;
    async fn search(
        &self,
        filter: &RoleSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Role>, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, String>;
    async fn create(&self, role: NewRole) -> Result<Role, String>;
    async fn update(&self, id: Uuid, role: UpdateRole) -> Result<Role, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Role>, String>;
    async fn assign_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String>;
    async fn revoke_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String>;
}

#[async_trait]
impl<T: RoleRepository + ?Sized + Send + Sync> RoleRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<Role>, String> {
        (**self).find_all().await
    }

    async fn search(
        &self,
        filter: &RoleSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Role>, String> {
        (**self).search(filter, sort_by, cursor, limit).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, String> {
        (**self).find_by_id(id).await
    }

    async fn create(&self, role: NewRole) -> Result<Role, String> {
        (**self).create(role).await
    }

    async fn update(&self, id: Uuid, role: UpdateRole) -> Result<Role, String> {
        (**self).update(id, role).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        (**self).delete(id).await
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Role>, String> {
        (**self).find_by_user_id(user_id).await
    }

    async fn assign_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String> {
        (**self).assign_to_user(user_id, role_id).await
    }

    async fn revoke_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String> {
        (**self).revoke_from_user(user_id, role_id).await
    }
}
