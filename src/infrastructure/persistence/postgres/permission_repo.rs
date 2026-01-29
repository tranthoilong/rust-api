use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::permission::{NewPermission, Permission, UpdatePermission};
use crate::domain::repositories::permission_repository::PermissionRepository;

pub struct PgPermissionRepository {
    pool: Pool<Postgres>,
}

impl PgPermissionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PermissionRepository for PgPermissionRepository {
    async fn find_all(&self) -> Result<Vec<Permission>, String> {
        sqlx::query_as!(
            Permission,
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM permissions"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Permission>, String> {
        sqlx::query_as!(
            Permission,
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM permissions WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, permission: NewPermission) -> Result<Permission, String> {
        sqlx::query_as!(
            Permission,
            r#"
            INSERT INTO permissions (name)
            VALUES ($1)
            RETURNING id, name, created_at, updated_at, deleted_at
            "#,
            permission.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update(&self, id: i32, permission: UpdatePermission) -> Result<Permission, String> {
        sqlx::query_as!(
            Permission,
            r#"
            UPDATE permissions
            SET name = COALESCE($1, name),
                updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, created_at, updated_at, deleted_at
            "#,
            permission.name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        let result = sqlx::query!(
            "UPDATE permissions SET deleted_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Permission not found".to_string());
        }

        Ok(())
    }
}
