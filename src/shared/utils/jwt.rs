use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenWithExpiration {
    pub token: String,
    pub expires_at: i64,
}

pub fn create_jwt(
    user_id: &str,
    secret: &[u8],
    duration: Duration,
) -> Result<TokenWithExpiration, String> {
    let expiration = Utc::now()
        .checked_add_signed(duration)
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        iat: Utc::now().timestamp() as usize,
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| e.to_string())?;

    Ok(TokenWithExpiration {
        token,
        expires_at: expiration,
    })
}

pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}
