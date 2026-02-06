use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    application::post::{
        bulk_delete_posts::{BulkDeletePostsInput, BulkDeletePostsUseCase},
        create_post::{CreatePostInput, CreatePostUseCase},
        delete_post::DeletePostUseCase,
        get_post_by_slug::GetPostBySlugUseCase,
        get_posts::GetPostsUseCase,
        update_post::{UpdatePostInput, UpdatePostUseCase},
    },
    interface::http::response::ApiResponse,
};

pub async fn list_posts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetPostsUseCase::new(state.post_repo.clone());

    match usecase.execute().await {
        Ok(posts) => ApiResponse::success(serde_json::json!(posts), None).into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn get_post_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let usecase = GetPostBySlugUseCase::new(state.post_repo.clone());

    match usecase.execute(&slug).await {
        Ok(Some(post)) => ApiResponse::success(serde_json::json!(post), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Post not found".to_string(),
            None,
            None,
        )
        .into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePostInput>,
) -> impl IntoResponse {
    let usecase = CreatePostUseCase::new(state.post_repo.clone());

    match usecase.execute(payload).await {
        Ok(post) => ApiResponse::created(serde_json::json!(post), None).into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostInput>,
) -> impl IntoResponse {
    let usecase = UpdatePostUseCase::new(state.post_repo.clone());

    match usecase.execute(id, payload).await {
        Ok(post) => ApiResponse::success(serde_json::json!(post), None).into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeletePostUseCase::new(state.post_repo.clone());

    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Post deleted".to_string()))
            .into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn bulk_delete_posts(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BulkDeletePostsInput>,
) -> impl IntoResponse {
    let usecase = BulkDeletePostsUseCase::new(state.post_repo.clone());

    match usecase.execute(payload).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Posts deleted".to_string()))
            .into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}
