use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    app::state::AppState,
    application::category::{get_categories::GetCategoriesUseCase, get_category::GetCategoryUseCase},
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

