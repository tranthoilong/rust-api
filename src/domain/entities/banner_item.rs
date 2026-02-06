use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BannerItem {
    pub id: Uuid,
    pub banner_id: Uuid,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub position: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

