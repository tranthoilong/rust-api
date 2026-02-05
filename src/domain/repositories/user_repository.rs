use crate::domain::entities::user::{NewUser, UpdateUser, User};
use crate::shared::utils::query::PaginatedResult;
use async_trait::async_trait;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserSearchFilter {
    pub search: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_all(&self) -> Result<Vec<User>, String>;
    async fn search(
        &self,
        filter: &UserSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<User>, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;

    async fn create(&self, user: NewUser) -> Result<User, String>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}

#[async_trait]
impl<T: UserRepository + ?Sized + Send + Sync> UserRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<User>, String> {
        (**self).find_all().await
    }

    async fn search(
        &self,
        filter: &UserSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<User>, String> {
        (**self).search(filter, sort_by, cursor, limit).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        (**self).find_by_email(email).await
    }

    async fn create(&self, user: NewUser) -> Result<User, String> {
        (**self).create(user).await
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, String> {
        (**self).update(id, user).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        (**self).delete(id).await
    }
}
