use serde::Deserialize;

use crate::{
    domain::repositories::user_repository::UserRepository,
    shared::utils::{hash::verify_password, jwt::create_jwt},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct LoginUseCase<R: UserRepository> {
    repo: R,
}

use chrono::Duration;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginOutput {
    pub access_token: String,
    pub access_token_expires_at: i64,
    pub refresh_token: String,
    pub refresh_token_expires_at: i64,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<LoginOutput, String> {
        let user = self
            .repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| "User not found".to_string())?;

        let is_valid = verify_password(&req.password, &user.password)?;

        if !is_valid {
            return Err("Invalid credentials".to_string());
        }

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let secret_bytes = secret.as_bytes();

        // Access Token: 1 hour
        let access_token_data = create_jwt(&user.id.to_string(), secret_bytes, Duration::hours(1))?;

        // Refresh Token: 7 days
        let refresh_token_data = create_jwt(&user.id.to_string(), secret_bytes, Duration::days(7))?;

        Ok(LoginOutput {
            access_token: access_token_data.token,
            access_token_expires_at: access_token_data.expires_at,
            refresh_token: refresh_token_data.token,
            refresh_token_expires_at: refresh_token_data.expires_at,
        })
    }
}
