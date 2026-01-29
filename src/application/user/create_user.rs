use crate::domain::{
    entities::user::{NewUser, User},
    repositories::user_repository::UserRepository,
};

pub struct CreateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, mut user: NewUser) -> Result<User, String> {
        user.password = crate::shared::utils::hash::hash_password(&user.password)?;
        self.repo.create(user).await
    }
}
