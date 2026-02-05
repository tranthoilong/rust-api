use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

pub struct GetUserUseCase<R: UserRepository> {
    repo: R,
}

use uuid::Uuid;

impl<R: UserRepository> GetUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<User>, String> {
        self.repo.find_by_id(id).await
    }
}
