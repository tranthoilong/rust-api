use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::state::AppState,
    application::category::{
        bulk_delete_categories::{BulkDeleteCategoriesInput, BulkDeleteCategoriesUseCase},
        create_category::{CreateCategoryInput, CreateCategoryUseCase},
        delete_category::DeleteCategoryUseCase,
        get_categories::GetCategoriesUseCase,
        get_category::GetCategoryUseCase,
        update_category::{UpdateCategoryInput, UpdateCategoryUseCase},
    },
    interface::http::response::ApiResponse,
};

#[derive(Deserialize)]
pub struct CategoryQuery {
    pub r#type: Option<String>,
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CategoryQuery>,
) -> impl IntoResponse {
    let usecase = GetCategoriesUseCase::new(state.category_repo.clone());
    let r#type = query.r#type.unwrap_or_else(|| "post".to_string());

    match usecase.execute(&r#type).await {
        Ok(cats) => ApiResponse::success(serde_json::json!(cats), None).into_response(),
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

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Query(query): Query<CategoryQuery>,
) -> impl IntoResponse {
    let usecase = GetCategoryUseCase::new(state.category_repo.clone());
    let r#type = query.r#type.unwrap_or_else(|| "post".to_string());

    match usecase.execute(&slug, &r#type).await {
        Ok(Some(cat)) => ApiResponse::success(serde_json::json!(cat), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Category not found".to_string(),
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

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryInput>,
) -> impl IntoResponse {
    let usecase = CreateCategoryUseCase::new(state.category_repo.clone());

    match usecase.execute(payload).await {
        Ok(cat) => ApiResponse::created(serde_json::json!(cat), None).into_response(),
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

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryInput>,
) -> impl IntoResponse {
    let usecase = UpdateCategoryUseCase::new(state.category_repo.clone());

    match usecase.execute(id, payload).await {
        Ok(cat) => ApiResponse::success(serde_json::json!(cat), None).into_response(),
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

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeleteCategoryUseCase::new(state.category_repo.clone());

    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Category deleted".to_string()))
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

pub async fn bulk_delete_categories(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BulkDeleteCategoriesInput>,
) -> impl IntoResponse {
    let usecase = BulkDeleteCategoriesUseCase::new(state.category_repo.clone());

    match usecase.execute(payload).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Categories deleted".to_string()))
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

