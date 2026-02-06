use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;

use crate::{
    app::state::AppState,
    application::settings::{
        get_setting::GetSettingUseCase, get_settings::GetSettingsUseCase,
        update_setting::UpdateSettingUseCase,
    },
    interface::http::response::ApiResponse,
};

pub async fn list_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetSettingsUseCase::new(state.setting_repo.clone());

    match usecase.execute().await {
        Ok(settings) => ApiResponse::success(serde_json::json!(settings), None).into_response(),
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

pub async fn get_setting(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let usecase = GetSettingUseCase::new(state.setting_repo.clone());

    match usecase.execute(&key).await {
        Ok(Some(setting)) => ApiResponse::success(serde_json::json!(setting), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Setting not found".to_string(),
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

#[derive(serde::Deserialize)]
pub struct UpdateSettingPayload {
    pub value: Option<Value>,
}

pub async fn update_setting(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(payload): Json<UpdateSettingPayload>,
) -> impl IntoResponse {
    let usecase = UpdateSettingUseCase::new(state.setting_repo.clone());

    match usecase.execute(&key, payload.value).await {
        Ok(setting) => ApiResponse::success(serde_json::json!(setting), None).into_response(),
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

