use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app::state::AppState,
    application::language::{
        get_default_language::GetDefaultLanguageUseCase, get_languages::GetLanguagesUseCase,
    },
    interface::http::response::ApiResponse,
};

pub async fn list_languages(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetLanguagesUseCase::new(state.language_repo.clone());

    match usecase.execute().await {
        Ok(languages) => ApiResponse::success(serde_json::json!(languages), None).into_response(),
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

pub async fn get_default_language(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetDefaultLanguageUseCase::new(state.language_repo.clone());

    match usecase.execute().await {
        Ok(Some(lang)) => ApiResponse::success(serde_json::json!(lang), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Default language not found".to_string(),
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
