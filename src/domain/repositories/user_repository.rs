use crate::domain::entities::user::{NewUser, UpdateUser, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;

    async fn create(&self, user: NewUser) -> Result<User, String>;
    async fn update(&self, id: i32, user: UpdateUser) -> Result<User, String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
}

#[async_trait]
impl<T: UserRepository + ?Sized + Send + Sync> UserRepository for std::sync::Arc<T> {
    async fn find_all(&self) -> Result<Vec<User>, String> {
        (**self).find_all().await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        (**self).find_by_email(email).await
    }

    async fn create(&self, user: NewUser) -> Result<User, String> {
        (**self).create(user).await
    }

    async fn update(&self, id: i32, user: UpdateUser) -> Result<User, String> {
        (**self).update(id, user).await
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        (**self).delete(id).await
    }
}
