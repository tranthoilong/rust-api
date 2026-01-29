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

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<String, String> {
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
        let token = create_jwt(&user.id.to_string(), secret.as_bytes())?;

        Ok(token)
    }
}
