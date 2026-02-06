use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::state::AppState,
    application::tag::{
        bulk_delete_tags::{BulkDeleteTagsInput, BulkDeleteTagsUseCase},
        create_tag::{CreateTagInput, CreateTagUseCase},
        delete_tag::DeleteTagUseCase,
        get_tag::GetTagUseCase,
        get_tags::GetTagsUseCase,
        update_tag::{UpdateTagInput, UpdateTagUseCase},
    },
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

pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTagInput>,
) -> impl IntoResponse {
    let usecase = CreateTagUseCase::new(state.tag_repo.clone());

    match usecase.execute(payload).await {
        Ok(tag) => ApiResponse::created(serde_json::json!(tag), None).into_response(),
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

pub async fn update_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTagInput>,
) -> impl IntoResponse {
    let usecase = UpdateTagUseCase::new(state.tag_repo.clone());

    match usecase.execute(id, payload).await {
        Ok(tag) => ApiResponse::success(serde_json::json!(tag), None).into_response(),
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

pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeleteTagUseCase::new(state.tag_repo.clone());

    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Tag deleted".to_string()))
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

pub async fn bulk_delete_tags(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BulkDeleteTagsInput>,
) -> impl IntoResponse {
    let usecase = BulkDeleteTagsUseCase::new(state.tag_repo.clone());

    match usecase.execute(payload).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Tags deleted".to_string()))
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
