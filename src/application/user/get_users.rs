use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::entities::user::User;

pub struct GetUsersUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUsersUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, String> {
        self.repo.find_all().await
    }
}
