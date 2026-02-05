use crate::app::state::AppState;
use crate::shared::utils::jwt::Claims;
use axum::{
    Extension,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Middleware "Decorator" like check.
/// Dùng để chặn request nếu user không có role cụ thể.
/// Ví dụ: Chỉ cho phép Admin.
pub async fn require_admin_role(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // 1. Lấy userId từ claims
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Invalid user ID in token".to_string(),
        )
    })?;

    // 2. Check DB xem user có role admin không
    match state.role_repo.find_by_user_id(user_id).await {
        Ok(roles) => {
            // Role seed đang dùng tên thường "admin"
            let is_admin = roles.iter().any(|r| r.name.eq_ignore_ascii_case("admin"));

            if is_admin {
                Ok(next.run(req).await)
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    "Forbidden: Requires Admin role".to_string(),
                ))
            }
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to verify roles".to_string(),
        )),
    }
}
