use crate::domain::{
    entities::media::Media,
    repositories::media_repository::MediaRepository,
};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateMediaInput {
    pub media_type: Option<String>,
    pub file_path: Option<String>,
}

pub struct UpdateMediaUseCase<R: MediaRepository> {
    repo: R,
}

impl<R: MediaRepository> UpdateMediaUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, input: UpdateMediaInput) -> Result<Media, String> {
        let existing = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Media not found".to_string())?;

        let media = Media {
            id: existing.id,
            user_id: existing.user_id,
            media_type: input.media_type.unwrap_or(existing.media_type),
            file_path: input.file_path.unwrap_or(existing.file_path),
            created_at: existing.created_at,
            updated_at: existing.updated_at,
            deleted_at: existing.deleted_at,
        };

        self.repo.update(media).await
    }
}

