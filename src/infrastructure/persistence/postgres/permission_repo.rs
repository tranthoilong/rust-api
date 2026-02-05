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

use uuid::Uuid;

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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, String> {
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

    async fn update(&self, id: Uuid, permission: UpdatePermission) -> Result<Permission, String> {
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

    async fn delete(&self, id: Uuid) -> Result<(), String> {
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

    async fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<Permission>, String> {
        sqlx::query_as!(
            Permission,
            r#"
            SELECT p.id, p.name, p.created_at, p.updated_at, p.deleted_at
            FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1 AND p.deleted_at IS NULL AND rp.deleted_at IS NULL
            "#,
            role_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn assign_to_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        let existing = sqlx::query!(
            r#"SELECT id, deleted_at FROM role_permissions WHERE role_id = $1 AND permission_id = $2 ORDER BY created_at DESC LIMIT 1"#,
            role_id,
            permission_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(record) = existing {
            if record.deleted_at.is_none() {
                return Ok(());
            }
            sqlx::query!(
                r#"UPDATE role_permissions SET deleted_at = NULL, updated_at = NOW() WHERE id = $1"#,
                record.id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            sqlx::query!(
                r#"INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)"#,
                role_id,
                permission_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn revoke_from_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"UPDATE role_permissions SET deleted_at = NOW() WHERE role_id = $1 AND permission_id = $2 AND deleted_at IS NULL"#,
            role_id,
            permission_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
