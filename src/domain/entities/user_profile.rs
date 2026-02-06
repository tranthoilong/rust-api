use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub bio: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserProfile {
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub bio: Option<String>,
}
