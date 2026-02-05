use crate::domain::repositories::media_repository::MediaRepository;
use uuid::Uuid;

pub struct DeleteMediaUseCase<R: MediaRepository> {
    repo: R,
}

impl<R: MediaRepository> DeleteMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), String> {
        self.repo.soft_delete(id).await
    }
}

