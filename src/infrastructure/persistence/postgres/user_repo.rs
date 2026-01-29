use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::user::{NewUser, UpdateUser, User, UserStatus};
use crate::domain::repositories::user_repository::UserRepository;

pub struct PgUserRepository {
    pool: Pool<Postgres>,
}

impl PgUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, String> {
        sqlx::query_as!(
            User,
            r#"SELECT id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, String> {
        sqlx::query_as!(
            User,
            r#"SELECT id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        sqlx::query_as!(
            User,
            r#"SELECT id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, user: NewUser) -> Result<User, String> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at
            "#,
            user.name,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update(&self, id: i32, user: UpdateUser) -> Result<User, String> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = COALESCE($1, name),
                email = COALESCE($2, email),
                password = COALESCE($3, password),
                status = COALESCE($4, status),
                updated_at = NOW()
            WHERE id = $5
            RETURNING id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at
            "#,
            user.name,
            user.email,
            user.password,
            user.status as Option<UserStatus>,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        // Soft delete
        let result = sqlx::query!(
            "UPDATE users SET deleted_at = NOW(), status = 'deleted' WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("User not found".to_string());
        }

        Ok(())
    }
}
