use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};

use crate::domain::entities::user::{NewUser, UpdateUser, User, UserStatus};
use crate::domain::repositories::user_repository::{UserRepository, UserSearchFilter};
use crate::shared::utils::query::{
    build_query, encode_cursor_text, encode_cursor_ts, BindValue, FieldInfo, FieldType,
    ListParams, PaginatedResult, SortDirection,
};

pub struct PgUserRepository {
    pool: Pool<Postgres>,
}

impl PgUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

use uuid::Uuid;

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

    async fn search(
        &self,
        filter: &UserSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<User>, String> {
        let allowed_fields = [
            FieldInfo {
                name: "name",
                field_type: FieldType::Text,
            },
            FieldInfo {
                name: "email",
                field_type: FieldType::Text,
            },
            FieldInfo {
                name: "created_at",
                field_type: FieldType::Timestamp,
            },
        ];

        let base_sql = r#"SELECT id, name, email, password, status as "status: UserStatus", created_at, updated_at, deleted_at FROM users"#;

        let params = ListParams {
            search: filter.search.clone(),
            fields: None,
            sort_by,
            cursor,
            limit: Some(limit),
        };

        let built = build_query(
            base_sql,
            &params,
            &allowed_fields,
            "created_at",
            SortDirection::Desc,
            &["name", "email"],
        )?;

        let mut query = sqlx::query_as::<_, User>(&built.sql);
        for b in built.binds {
            query = match b {
                BindValue::Text(v) => query.bind(v),
                BindValue::Timestamp(v) => query.bind(v),
                BindValue::Uuid(v) => query.bind(v),
                BindValue::I64(v) => query.bind(v),
            };
        }

        let items = query.fetch_all(&self.pool).await.map_err(|e| e.to_string())?;

        let next_cursor = if items.len() as i64 == built.limit {
            if let Some(last) = items.last() {
                match built.sort_field {
                    "name" => Some(encode_cursor_text(&last.name, last.id)),
                    "email" => Some(encode_cursor_text(&last.email, last.id)),
                    "created_at" => last
                        .created_at
                        .map(|dt| encode_cursor_ts(dt, last.id)),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(PaginatedResult {
            items,
            next_cursor,
            limit: built.limit,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
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
        // Decode row thủ công để ép id là UUID (tránh lỗi decode 8 vs 16 bytes từ cache/macro).
        let row = sqlx::query(
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, password, status, created_at, updated_at, deleted_at
            "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
        let name: String = row.try_get("name").map_err(|e| e.to_string())?;
        let email: String = row.try_get("email").map_err(|e| e.to_string())?;
        let password: String = row.try_get("password").map_err(|e| e.to_string())?;
        let status: Option<UserStatus> = row.try_get("status").map_err(|e| e.to_string())?;
        let created_at: Option<chrono::NaiveDateTime> = row.try_get("created_at").map_err(|e| e.to_string())?;
        let updated_at: Option<chrono::NaiveDateTime> = row.try_get("updated_at").map_err(|e| e.to_string())?;
        let deleted_at: Option<chrono::NaiveDateTime> = row.try_get("deleted_at").map_err(|e| e.to_string())?;

        Ok(User {
            id,
            name,
            email,
            password,
            status,
            created_at,
            updated_at,
            deleted_at,
        })
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, String> {
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

    async fn delete(&self, id: Uuid) -> Result<(), String> {
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
