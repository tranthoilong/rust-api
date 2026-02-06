use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app::state::AppState, application::audit_log::get_audit_logs::GetAuditLogsUseCase,
    domain::repositories::audit_log_repository::AuditLogFilter,
    interface::http::response::ApiResponse,
};

pub async fn list_audit_logs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Tạm thời không filter, trả về một số bản ghi mới nhất
    let filter = AuditLogFilter {
        user_id: None,
        entity_type: None,
        entity_id: None,
    };

    let usecase = GetAuditLogsUseCase::new(state.audit_log_repo.clone());

    match usecase.execute(filter).await {
        Ok(logs) => ApiResponse::success(serde_json::json!(logs), None).into_response(),
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
