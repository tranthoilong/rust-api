use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::state::AppState,
    application::banner::{
        bulk_delete_banners::BulkDeleteBannersInput,
        bulk_delete_banners::BulkDeleteBannersUseCase,
        create_banner::CreateBannerInput,
        create_banner::CreateBannerUseCase,
        create_banner_item::CreateBannerItemInput,
        create_banner_item::CreateBannerItemUseCase,
        delete_banner::DeleteBannerUseCase,
        delete_banner_item::DeleteBannerItemUseCase,
        get_banner_by_key::GetBannerByKeyUseCase,
        list_active_banners::ListActiveBannersUseCase,
        update_banner::UpdateBannerInput,
        update_banner::UpdateBannerUseCase,
        update_banner_item::UpdateBannerItemInput,
        update_banner_item::UpdateBannerItemUseCase,
    },
    interface::http::response::ApiResponse,
};
use uuid::Uuid;

pub async fn list_active_banners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let usecase = ListActiveBannersUseCase::new(state.banner_repo.clone());

    match usecase.execute().await {
        Ok(banners) => ApiResponse::success(serde_json::json!(banners), None).into_response(),
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

pub async fn get_banner_by_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let usecase = GetBannerByKeyUseCase::new(state.banner_repo.clone());

    match usecase.execute(&key).await {
        Ok(Some(data)) => ApiResponse::success(serde_json::json!(data), None).into_response(),
        Ok(None) => ApiResponse::<()>::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
            "Banner not found".to_string(),
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

pub async fn create_banner(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateBannerInput>,
) -> impl IntoResponse {
    let usecase = CreateBannerUseCase::new(state.banner_repo.clone());
    match usecase.execute(payload).await {
        Ok(banner) => ApiResponse::success(serde_json::json!(banner), Some("Banner created".to_string()))
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

pub async fn update_banner(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBannerInput>,
) -> impl IntoResponse {
    let usecase = UpdateBannerUseCase::new(state.banner_repo.clone());
    match usecase.execute(id, payload).await {
        Ok(banner) => ApiResponse::success(serde_json::json!(banner), Some("Banner updated".to_string()))
            .into_response(),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            ApiResponse::<()>::error(status, "ERROR".to_string(), e, None, None).into_response()
        }
    }
}

pub async fn delete_banner(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeleteBannerUseCase::new(state.banner_repo.clone());
    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Banner deleted".to_string()))
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

pub async fn bulk_delete_banners(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BulkDeleteBannersInput>,
) -> impl IntoResponse {
    let usecase = BulkDeleteBannersUseCase::new(state.banner_repo.clone());
    match usecase.execute(payload).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Banners deleted".to_string()))
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

pub async fn create_banner_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateBannerItemInput>,
) -> impl IntoResponse {
    let usecase = CreateBannerItemUseCase::new(state.banner_repo.clone());
    match usecase.execute(payload).await {
        Ok(item) => ApiResponse::success(serde_json::json!(item), Some("Banner item created".to_string()))
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

pub async fn update_banner_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBannerItemInput>,
) -> impl IntoResponse {
    let usecase = UpdateBannerItemUseCase::new(state.banner_repo.clone());
    match usecase.execute(id, payload).await {
        Ok(item) => ApiResponse::success(serde_json::json!(item), Some("Banner item updated".to_string()))
            .into_response(),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            ApiResponse::<()>::error(status, "ERROR".to_string(), e, None, None).into_response()
        }
    }
}

pub async fn delete_banner_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let usecase = DeleteBannerItemUseCase::new(state.banner_repo.clone());
    match usecase.execute(id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({}), Some("Banner item deleted".to_string()))
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
