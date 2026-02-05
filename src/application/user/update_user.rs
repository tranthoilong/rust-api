use crate::domain::{
    entities::user::{UpdateUser, User},
    repositories::user_repository::UserRepository,
};

pub struct UpdateUserUseCase<R: UserRepository> {
    repo: R,
}

use uuid::Uuid;

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, user: UpdateUser) -> Result<User, String> {
        self.repo.update(id, user).await
    }
}
