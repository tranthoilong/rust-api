use axum::{Json, http::StatusCode};
use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<ApiErrorDetail>,
    pub pagination: Option<Value>,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub details: Value,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub struct Meta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: Option<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.unwrap_or_else(|| "Request successful".to_string()),
                data: Some(data),
                error: None,
                pagination: None,
                timestamp: Utc::now().to_rfc3339(),
                path: None,
            }),
        )
    }

    pub fn success_with_pagination(
        data: T,
        pagination: Value,
        message: Option<String>,
    ) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.unwrap_or_else(|| "Request successful".to_string()),
                data: Some(data),
                error: None,
                pagination: Some(pagination),
                timestamp: Utc::now().to_rfc3339(),
                path: None,
            }),
        )
    }

    pub fn created(data: T, message: Option<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::CREATED,
            Json(Self {
                success: true,
                message: message.unwrap_or_else(|| "Resource created successfully".to_string()),
                data: Some(data),
                error: None,
                pagination: None,
                timestamp: Utc::now().to_rfc3339(),
                path: None,
            }),
        )
    }

    pub fn error(
        status: StatusCode,
        code: String,
        message: String,
        details: Option<Value>,
        path: Option<String>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        (
            status,
            Json(ApiResponse {
                success: false,
                message: message.clone(),
                data: None,
                error: Some(ApiErrorDetail {
                    code,
                    details: details.unwrap_or_else(|| {
                        serde_json::json!({
                            "message": message,
                            "statusCode": status.as_u16()
                        })
                    }),
                }),
                pagination: None,
                timestamp: Utc::now().to_rfc3339(),
                path,
            }),
        )
    }
}

impl<T: Serialize> ApiResponse<PaginatedData<T>> {
    #[allow(dead_code)]
    pub fn success_paginated(
        data: Vec<T>,
        page: i64,
        limit: i64,
        total: i64,
    ) -> (StatusCode, Json<Self>) {
        let total_pages = if limit > 0 {
            (total as f64 / limit as f64).ceil() as i64
        } else {
            0
        };

        let paginated_data = PaginatedData {
            data,
            meta: Meta {
                total,
                page,
                limit,
                total_pages,
            },
        };

        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: "Request successful".to_string(),
                data: Some(paginated_data),
                error: None,
                pagination: None,
                timestamp: Utc::now().to_rfc3339(),
                path: None,
            }),
        )
    }
}

use axum::response::{IntoResponse, Response};

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
