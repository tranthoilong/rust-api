use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::role::{
    create_role::CreateRoleUseCase, delete_role::DeleteRoleUseCase, get_role::GetRoleUseCase,
    get_roles::GetRolesUseCase, update_role::UpdateRoleUseCase,
};
use crate::domain::entities::role::{NewRole, UpdateRole};
use crate::interface::http::response::ApiResponse;

pub async fn get_roles(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetRolesUseCase::new(state.role_repo.clone());
    match usecase.execute().await {
        Ok(roles) => {
            let data = roles.into_iter().map(|r| serde_json::json!(r)).collect();
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

pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewRole>,
) -> impl IntoResponse {
    let usecase = CreateRoleUseCase::new(state.role_repo.clone());
    match usecase.execute(payload).await {
        Ok(role) => ApiResponse::created(serde_json::json!(role), None).into_response(),
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

pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let usecase = GetRoleUseCase::new(state.role_repo.clone());
    match usecase.execute(id).await {
        Ok(Some(role)) => ApiResponse::success(serde_json::json!(role), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Role not found".to_string(),
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

pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRole>,
) -> impl IntoResponse {
    let usecase = UpdateRoleUseCase::new(state.role_repo.clone());
    match usecase.execute(id, payload).await {
        Ok(role) => ApiResponse::success(serde_json::json!(role), None).into_response(),
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

pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let usecase = DeleteRoleUseCase::new(state.role_repo.clone());
    match usecase.execute(id).await {
        Ok(_) => {
            ApiResponse::success((), Some("Role deleted successfully".to_string())).into_response()
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
