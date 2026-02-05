use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::state::AppState,
    application::banner::{
        get_banner_by_key::GetBannerByKeyUseCase, list_active_banners::ListActiveBannersUseCase,
    },
    interface::http::response::ApiResponse,
};

pub async fn list_active_banners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = ListActiveBannersUseCase::new(state.banner_repo.clone());

    match usecase.execute().await {
        Ok(banners) => ApiResponse::success(serde_json::json!(banners), None).into_response(),
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

pub async fn get_banner_by_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let usecase = GetBannerByKeyUseCase::new(state.banner_repo.clone());

    match usecase.execute(&key).await {
        Ok(Some(data)) => ApiResponse::success(serde_json::json!(data), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Banner not found".to_string(),
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

