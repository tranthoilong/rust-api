use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: i32,
    pub user_id: i32,
    pub media_type: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Media {
    pub fn generate_storage_path(user_id: i32, media_type: &str, filename: &str) -> String {
        let now = Utc::now();
        format!(
            "/{}/{}/{}/{}/{}/{}",
            user_id,
            media_type,
            now.format("%Y"),
            now.format("%m"),
            now.format("%d"),
            filename
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMedia {
    pub user_id: i32,
    pub media_type: String,
    pub file_path: String,
}
