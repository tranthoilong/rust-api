use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::permission::{
    assign_permission_to_role::AssignPermissionToRoleUseCase,
    get_permissions_by_role::GetPermissionsByRoleUseCase,
    revoke_permission_from_role::RevokePermissionFromRoleUseCase,
};
use crate::application::role::{
    create_role::CreateRoleUseCase, delete_role::DeleteRoleUseCase, get_role::GetRoleUseCase,
    get_roles::GetRolesUseCase, update_role::UpdateRoleUseCase,
};
use crate::domain::entities::role::{NewRole, UpdateRole};
use crate::interface::http::response::ApiResponse;
use crate::shared::utils::query::ListParams;

pub async fn get_roles(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let usecase = GetRolesUseCase::new(state.role_repo.clone());
    match usecase.execute(&params).await {
        Ok(result) => {
            let data = result
                .items
                .into_iter()
                .map(|r| serde_json::json!(r))
                .collect();
            let pagination = serde_json::json!({
                "next_cursor": result.next_cursor,
                "limit": result.limit,
                "sort_by": params.sort_by.clone(),
                "fields": params.fields.clone(),
                "search": params.search.clone()
            });
            ApiResponse::<Vec<serde_json::Value>>::success_with_pagination(
                data,
                pagination,
                None,
            )
            .into_response()
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

use uuid::Uuid;

pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
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
    Path(id): Path<Uuid>,
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
    Path(id): Path<Uuid>,
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

pub async fn get_role_permissions(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = GetPermissionsByRoleUseCase::new(state.permission_repo.clone());
    match usecase.execute(role_id).await {
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

pub async fn assign_permission(
    State(state): State<Arc<AppState>>,
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let usecase = AssignPermissionToRoleUseCase::new(state.permission_repo.clone());
    match usecase.execute(role_id, permission_id).await {
        Ok(_) => ApiResponse::success(
            (),
            Some("Permission assigned to role successfully".to_string()),
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

pub async fn revoke_permission(
    State(state): State<Arc<AppState>>,
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let usecase = RevokePermissionFromRoleUseCase::new(state.permission_repo.clone());
    match usecase.execute(role_id, permission_id).await {
        Ok(_) => ApiResponse::success(
            (),
            Some("Permission revoked from role successfully".to_string()),
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
