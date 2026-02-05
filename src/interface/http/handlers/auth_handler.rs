use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::user::{
    create_user::CreateUserUseCase,
    login_user::{LoginRequest, LoginUseCase},
};
use crate::domain::entities::user::NewUser;
use crate::interface::http::response::ApiResponse;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let usecase = LoginUseCase::new(state.user_repo.clone());

    match usecase.execute(payload).await {
        Ok(data) => ApiResponse::success(serde_json::json!(data), None).into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    let usecase = CreateUserUseCase::new(state.user_repo.clone());
    match usecase.execute(payload).await {
        Ok(user) => ApiResponse::created(serde_json::json!(user), None).into_response(),
        Err(e) => ApiResponse::<()>::error(
            StatusCode::BAD_REQUEST,
            "REGISTRATION_FAILED".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}

pub async fn me(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<crate::shared::utils::jwt::Claims>,
) -> impl IntoResponse {
    use crate::application::user::get_user::GetUserUseCase;
    let usecase = GetUserUseCase::new(state.user_repo.clone());

    // claims.sub is user_id as string
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST,
                "INVALID_TOKEN_SUB".to_string(),
                "Token subject is not a valid uuid".to_string(),
                None,
                None,
            )
            .into_response();
        }
    };

    match usecase.execute(user_id).await {
        Ok(Some(user)) => ApiResponse::success(serde_json::json!(user), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "USER_NOT_FOUND".to_string(),
            "User not found".to_string(),
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
