use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::application::role::{
    assign_role_to_user::AssignRoleToUserUseCase, revoke_role_from_user::RevokeRoleFromUserUseCase,
};
use crate::application::user::{
    create_user::CreateUserUseCase, delete_user::DeleteUserUseCase, get_user::GetUserUseCase,
    get_users::GetUsersUseCase, update_user::UpdateUserUseCase,
};
use crate::domain::entities::user::{NewUser, UpdateUser};
use crate::interface::http::response::ApiResponse;
use uuid::Uuid;

pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = GetUsersUseCase::new(state.user_repo.clone());

    match usecase.execute().await {
        Ok(users) => {
            let data = users.into_iter().map(|u| serde_json::json!(u)).collect();
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

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    let usecase = CreateUserUseCase::new(state.user_repo.clone());
    match usecase.execute(payload).await {
        Ok(user) => ApiResponse::created(serde_json::json!(user), None).into_response(),
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

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = GetUserUseCase::new(state.user_repo.clone());
    match usecase.execute(id).await {
        Ok(Some(user)) => ApiResponse::success(serde_json::json!(user), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "User not found".to_string(),
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

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let usecase = UpdateUserUseCase::new(state.user_repo.clone());
    match usecase.execute(id, payload).await {
        Ok(user) => ApiResponse::success(serde_json::json!(user), None).into_response(),
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

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeleteUserUseCase::new(state.user_repo.clone());
    match usecase.execute(id).await {
        Ok(_) => {
            ApiResponse::success((), Some("User deleted successfully".to_string())).into_response()
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

pub async fn assign_role(
    State(state): State<Arc<AppState>>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let usecase = AssignRoleToUserUseCase::new(state.role_repo.clone());
    match usecase.execute(user_id, role_id).await {
        Ok(_) => {
            ApiResponse::success((), Some("Role assigned successfully".to_string())).into_response()
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

pub async fn revoke_role(
    State(state): State<Arc<AppState>>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let usecase = RevokeRoleFromUserUseCase::new(state.role_repo.clone());
    match usecase.execute(user_id, role_id).await {
        Ok(_) => {
            ApiResponse::success((), Some("Role revoked successfully".to_string())).into_response()
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
