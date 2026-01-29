use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::user::login_user::{LoginRequest, LoginUseCase};
use crate::interface::http::response::ApiResponse;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let usecase = LoginUseCase::new(state.user_repo.clone());

    match usecase.execute(payload).await {
        Ok(token) => {
            ApiResponse::success(serde_json::json!({ "token": token }), None).into_response()
        }
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
