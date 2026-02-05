use crate::domain::repositories::user_repository::UserRepository;

pub struct DeleteUserUseCase<R: UserRepository> {
    repo: R,
}

use uuid::Uuid;

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.delete(id).await
    }
}
