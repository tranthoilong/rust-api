use crate::application::common::list_params::{ListParams, PaginatedResult};
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{UserRepository, UserSearchFilter};

pub struct GetUsersUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUsersUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: &ListParams) -> Result<PaginatedResult<User>, String> {
        let filter = UserSearchFilter {
            search: params.search.clone(),
        };
        let limit = params.limit.unwrap_or(20).clamp(1, 100);
        self.repo
            .search(&filter, params.sort_by.clone(), params.cursor.clone(), limit)
            .await
    }
}
