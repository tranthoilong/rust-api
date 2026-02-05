use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::shared::utils::query::{ListParams, PaginatedResult};

pub struct GetUsersUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUsersUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: &ListParams) -> Result<PaginatedResult<User>, String> {
        self.repo.find_paginated(params).await
    }
}
