use crate::domain::{entities::media::Media, repositories::media_repository::MediaRepository};

pub struct GetMediaUseCase<R: MediaRepository> {
    repo: R,
}

impl<R: MediaRepository> GetMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Media>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_by_user_id(&self, user_id: i32) -> Result<Vec<Media>, String> {
        self.repo.find_by_user_id(user_id).await
    }
}
