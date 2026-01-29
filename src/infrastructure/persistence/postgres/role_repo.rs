use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::role::{NewRole, Role, UpdateRole};
use crate::domain::repositories::role_repository::RoleRepository;

pub struct PgRoleRepository {
    pool: Pool<Postgres>,
}

impl PgRoleRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn find_all(&self) -> Result<Vec<Role>, String> {
        sqlx::query_as!(
            Role,
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM roles"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, String> {
        sqlx::query_as!(
            Role,
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM roles WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, role: NewRole) -> Result<Role, String> {
        sqlx::query_as!(
            Role,
            r#"
            INSERT INTO roles (name)
            VALUES ($1)
            RETURNING id, name, created_at, updated_at, deleted_at
            "#,
            role.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update(&self, id: i32, role: UpdateRole) -> Result<Role, String> {
        sqlx::query_as!(
            Role,
            r#"
            UPDATE roles
            SET name = COALESCE($1, name),
                updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, created_at, updated_at, deleted_at
            "#,
            role.name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        let result = sqlx::query!("UPDATE roles SET deleted_at = NOW() WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Role not found".to_string());
        }

        Ok(())
    }
}
