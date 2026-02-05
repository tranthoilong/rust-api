use crate::domain::{
    entities::media::{Media, NewMedia},
    repositories::media_repository::MediaRepository,
};

pub struct CreateMediaUseCase<R: MediaRepository> {
    repo: R,
}

impl<R: MediaRepository> CreateMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, media: NewMedia) -> Result<Media, String> {
        self.repo.create(media).await
    }
}
