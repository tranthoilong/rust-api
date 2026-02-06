use std::sync::Arc;

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::user_profile_repository::UserProfileRepository;
use uuid::Uuid;

pub struct GetProfileUseCase {
    repo: Arc<dyn UserProfileRepository>,
}

impl GetProfileUseCase {
    pub fn new(repo: Arc<dyn UserProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Option<UserProfile>, String> {
        self.repo.find_by_user_id(user_id).await
    }
}
