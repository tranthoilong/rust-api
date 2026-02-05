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

    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, String> {
        sqlx::query_as!(
            Role,
            r#"
            SELECT r.id, r.name, r.created_at, r.updated_at, r.deleted_at
            FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1 AND r.deleted_at IS NULL AND ur.deleted_at IS NULL
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn assign_to_user(&self, user_id: i32, role_id: i32) -> Result<(), String> {
        let existing = sqlx::query!(
            r#"SELECT id, deleted_at FROM user_roles WHERE user_id = $1 AND role_id = $2 ORDER BY created_at DESC LIMIT 1"#,
            user_id,
            role_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(record) = existing {
            if record.deleted_at.is_none() {
                return Ok(()); // Already assigned
            }
            // Restore
            sqlx::query!(
                r#"UPDATE user_roles SET deleted_at = NULL, updated_at = NOW() WHERE id = $1"#,
                record.id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            // Create
            sqlx::query!(
                r#"INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)"#,
                user_id,
                role_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn revoke_from_user(&self, user_id: i32, role_id: i32) -> Result<(), String> {
        sqlx::query!(
            r#"UPDATE user_roles SET deleted_at = NOW() WHERE user_id = $1 AND role_id = $2 AND deleted_at IS NULL"#,
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
