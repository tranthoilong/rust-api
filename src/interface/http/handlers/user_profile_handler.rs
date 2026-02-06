use std::sync::Arc;

use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::state::AppState,
    application::user_profile::{get_profile::GetProfileUseCase, update_profile::UpdateProfileUseCase},
    domain::entities::user_profile::UpdateUserProfile,
    interface::http::response::ApiResponse,
    shared::utils::jwt::Claims,
};

pub async fn get_me_profile(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                "Invalid user id".to_string(),
                None,
                None,
            )
            .into_response()
        }
    };

    let usecase = GetProfileUseCase::new(state.user_profile_repo.clone());

    match usecase.execute(user_id).await {
        Ok(Some(profile)) => ApiResponse::success(serde_json::json!(profile), None).into_response(),
        Ok(None) => ApiResponse::success(serde_json::json!(serde_json::json!({})), None)
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

pub async fn update_me_profile(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateUserProfile>,
) -> impl IntoResponse {
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                "Invalid user id".to_string(),
                None,
                None,
            )
            .into_response()
        }
    };

    let usecase = UpdateProfileUseCase::new(state.user_profile_repo.clone());

    match usecase.execute(user_id, payload).await {
        Ok(profile) => ApiResponse::success(serde_json::json!(profile), None).into_response(),
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

