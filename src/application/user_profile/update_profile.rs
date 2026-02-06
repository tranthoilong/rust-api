use std::sync::Arc;

use crate::domain::entities::user_profile::{UpdateUserProfile, UserProfile};
use crate::domain::repositories::user_profile_repository::UserProfileRepository;
use uuid::Uuid;

pub struct UpdateProfileUseCase {
    repo: Arc<dyn UserProfileRepository>,
}

impl UpdateProfileUseCase {
    pub fn new(repo: Arc<dyn UserProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        data: UpdateUserProfile,
    ) -> Result<UserProfile, String> {
        self.repo.upsert(user_id, data).await
    }
}
