use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::permission::{
    create_permission::CreatePermissionUseCase, delete_permission::DeletePermissionUseCase,
    get_permission::GetPermissionUseCase, get_permissions::GetPermissionsUseCase,
    update_permission::UpdatePermissionUseCase,
};
use crate::domain::entities::permission::{NewPermission, UpdatePermission};
use crate::interface::http::response::ApiResponse;

pub async fn get_permissions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetPermissionsUseCase::new(state.permission_repo.clone());
    match usecase.execute().await {
        Ok(permissions) => {
            let data = permissions
                .into_iter()
                .map(|p| serde_json::json!(p))
                .collect();
            ApiResponse::<Vec<serde_json::Value>>::success(data, None).into_response()
        }
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

pub async fn create_permission(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewPermission>,
) -> impl IntoResponse {
    let usecase = CreatePermissionUseCase::new(state.permission_repo.clone());
    match usecase.execute(payload).await {
        Ok(permission) => ApiResponse::created(serde_json::json!(permission), None).into_response(),
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

pub async fn get_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let usecase = GetPermissionUseCase::new(state.permission_repo.clone());
    match usecase.execute(id).await {
        Ok(Some(permission)) => {
            ApiResponse::success(serde_json::json!(permission), None).into_response()
        }
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Permission not found".to_string(),
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

pub async fn update_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermission>,
) -> impl IntoResponse {
    let usecase = UpdatePermissionUseCase::new(state.permission_repo.clone());
    match usecase.execute(id, payload).await {
        Ok(permission) => ApiResponse::success(serde_json::json!(permission), None).into_response(),
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

pub async fn delete_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let usecase = DeletePermissionUseCase::new(state.permission_repo.clone());
    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success((), Some("Permission deleted successfully".to_string()))
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
