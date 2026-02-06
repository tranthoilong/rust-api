use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::user_profile::{UpdateUserProfile, UserProfile};
use crate::domain::repositories::user_profile_repository::UserProfileRepository;

pub struct PgUserProfileRepository {
    pool: Pool<Postgres>,
}

impl PgUserProfileRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProfileRepository for PgUserProfileRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, String> {
        sqlx::query_as!(
            UserProfile,
            r#"
            SELECT id, user_id, avatar_url, phone, address, bio,
                   created_at, updated_at, deleted_at
            FROM user_profiles
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn upsert(&self, user_id: Uuid, data: UpdateUserProfile) -> Result<UserProfile, String> {
        sqlx::query_as!(
            UserProfile,
            r#"
            INSERT INTO user_profiles (user_id, avatar_url, phone, address, bio)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id) DO UPDATE
            SET avatar_url = COALESCE($2, user_profiles.avatar_url),
                phone      = COALESCE($3, user_profiles.phone),
                address    = COALESCE($4, user_profiles.address),
                bio        = COALESCE($5, user_profiles.bio),
                updated_at = NOW(),
                deleted_at = NULL
            RETURNING id, user_id, avatar_url, phone, address, bio,
                      created_at, updated_at, deleted_at
            "#,
            user_id,
            data.avatar_url,
            data.phone,
            data.address,
            data.bio
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
