use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    app::state::AppState,
    application::tag::{get_tag::GetTagUseCase, get_tags::GetTagsUseCase},
    interface::http::response::ApiResponse,
};

#[derive(Deserialize)]
pub struct TagQuery {
    pub r#type: Option<String>,
}

pub async fn list_tags(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TagQuery>,
) -> impl IntoResponse {
    let usecase = GetTagsUseCase::new(state.tag_repo.clone());
    let r#type = query.r#type.unwrap_or_else(|| "post".to_string());

    match usecase.execute(&r#type).await {
        Ok(tags) => ApiResponse::success(serde_json::json!(tags), None).into_response(),
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

pub async fn get_tag(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Query(query): Query<TagQuery>,
) -> impl IntoResponse {
    let usecase = GetTagUseCase::new(state.tag_repo.clone());
    let r#type = query.r#type.unwrap_or_else(|| "post".to_string());

    match usecase.execute(&slug, &r#type).await {
        Ok(Some(tag)) => ApiResponse::success(serde_json::json!(tag), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Tag not found".to_string(),
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

