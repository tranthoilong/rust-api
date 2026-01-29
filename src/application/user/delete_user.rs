use crate::domain::repositories::user_repository::UserRepository;

pub struct DeleteUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i32) -> Result<(), String> {
        self.repo.delete(id).await
    }
}
