use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entities::permission::{NewPermission, Permission, UpdatePermission};
use crate::domain::repositories::permission_repository::{
    PermissionRepository, PermissionSearchFilter,
};
use crate::shared::utils::query::{
    BindValue, FieldInfo, FieldType, ListParams, PaginatedResult, SortDirection, build_query,
    encode_cursor_text, encode_cursor_ts,
};

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
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM permissions WHERE deleted_at IS NULL"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn search(
        &self,
        filter: &PermissionSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Permission>, String> {
        let allowed_fields = [
            FieldInfo {
                name: "name",
                field_type: FieldType::Text,
            },
            FieldInfo {
                name: "created_at",
                field_type: FieldType::Timestamp,
            },
        ];

        let base_sql = r#"SELECT id, name, created_at, updated_at, deleted_at FROM permissions WHERE deleted_at IS NULL"#;

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
            &["name"],
        )?;

        let mut query = sqlx::query_as::<_, Permission>(&built.sql);
        for b in built.binds {
            query = match b {
                BindValue::Text(v) => query.bind(v),
                BindValue::Timestamp(v) => query.bind(v),
                BindValue::Uuid(v) => query.bind(v),
                BindValue::I64(v) => query.bind(v),
            };
        }

        let items = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        let next_cursor = if items.len() as i64 == built.limit {
            if let Some(last) = items.last() {
                match built.sort_field {
                    "name" => Some(encode_cursor_text(&last.name, last.id)),
                    "created_at" => last.created_at.map(|dt| encode_cursor_ts(dt, last.id)),
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, String> {
        sqlx::query_as!(
            Permission,
            r#"SELECT id, name, created_at, updated_at, deleted_at FROM permissions WHERE id = $1 AND deleted_at IS NULL"#,
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
        // Validate that there's at least something to update
        if permission.name.is_none() {
            return Err("No fields to update".to_string());
        }

        sqlx::query_as!(
            Permission,
            r#"
            UPDATE permissions
            SET name = COALESCE($1, name),
                updated_at = NOW()
            WHERE id = $2 AND deleted_at IS NULL
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
