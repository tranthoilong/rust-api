use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::state::AppState,
    application::post::{get_post_by_slug::GetPostBySlugUseCase, get_posts::GetPostsUseCase},
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

