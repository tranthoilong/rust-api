use crate::domain::entities::user_profile::{UpdateUserProfile, UserProfile};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, String>;
    async fn upsert(&self, user_id: Uuid, data: UpdateUserProfile) -> Result<UserProfile, String>;
}

#[async_trait]
impl<T: UserProfileRepository + ?Sized + Send + Sync> UserProfileRepository for std::sync::Arc<T> {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, String> {
        (**self).find_by_user_id(user_id).await
    }

    async fn upsert(&self, user_id: Uuid, data: UpdateUserProfile) -> Result<UserProfile, String> {
        (**self).upsert(user_id, data).await
    }
}
