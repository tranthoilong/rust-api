use axum::{
    extract::{Extension, Multipart, Path as AxumPath, State},
    http::StatusCode,
    response::Json,
};
use std::path::Path;
use std::sync::Arc;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    app::state::AppState,
    application::media::{create_media::CreateMediaUseCase, get_media::GetMediaUseCase},
    domain::entities::media::{Media, NewMedia},
    shared::utils::jwt::Claims,
};

const UPLOAD_ROOT: &str = "uploads";

pub async fn upload_media(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<Media>, (StatusCode, String)> {
    let user_id = claims.sub.parse::<i32>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Invalid user ID in token".to_string(),
        )
    })?;

    // We only process the first field for now
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)))?
    {
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let filename = field.file_name().unwrap_or("unknown_file").to_string();

        // Determine media_type bucket
        let media_type = if content_type.starts_with("image/") {
            "images"
        } else if content_type.starts_with("video/") {
            "videos"
        } else {
            "documents"
        };

        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read field bytes: {}", e),
            )
        })?;

        // Generate custom path
        let storage_path = Media::generate_storage_path(user_id, media_type, &filename);
        let full_path = Path::new(UPLOAD_ROOT).join(storage_path.trim_start_matches('/'));

        // Ensure directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create directory: {}", e),
                )
            })?;
        }

        // Save file
        let mut file = fs::File::create(&full_path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create file: {}", e),
            )
        })?;
        file.write_all(&data).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write to file: {}", e),
            )
        })?;

        // Create UseCase and execute
        let use_case = CreateMediaUseCase::new(state.media_repo.clone());
        let new_media = NewMedia {
            user_id,
            media_type: media_type.to_string(),
            file_path: storage_path,
        };

        let result = use_case.execute(new_media).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

        return Ok(Json(result));
    }

    Err((StatusCode::BAD_REQUEST, "No file uploaded".to_string()))
}

pub async fn get_media(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<i32>,
) -> Result<Json<Media>, (StatusCode, String)> {
    let use_case = GetMediaUseCase::new(state.media_repo.clone());

    match use_case.get_by_id(id).await {
        Ok(Some(media)) => Ok(Json(media)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Media not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn get_user_media(
    State(state): State<Arc<AppState>>,
    AxumPath(user_id): AxumPath<i32>,
) -> Result<Json<Vec<Media>>, (StatusCode, String)> {
    let use_case = GetMediaUseCase::new(state.media_repo.clone());

    match use_case.get_by_user_id(user_id).await {
        Ok(media_list) => Ok(Json(media_list)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}
