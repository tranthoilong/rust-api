use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(auth_header) if auth_header.starts_with("Bearer ") => {
            let token = &auth_header[7..];
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

            match crate::shared::utils::jwt::verify_jwt(token, secret.as_bytes()) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
            }
        }
        _ => Err((
            StatusCode::UNAUTHORIZED,
            "Missing or invalid token".to_string(),
        )),
    }
}
