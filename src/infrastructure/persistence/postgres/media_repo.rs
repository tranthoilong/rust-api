use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::media::{Media, NewMedia};
use crate::domain::repositories::media_repository::{MediaRepository, MediaSearchFilter};
use crate::shared::utils::query::{
    build_query, build_query_with_seed, encode_cursor_text, encode_cursor_ts, BindValue, FieldInfo,
    FieldType, ListParams, PaginatedResult, SortDirection,
};

pub struct PgMediaRepository {
    pool: Pool<Postgres>,
}

impl PgMediaRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

use uuid::Uuid;

#[async_trait]
impl MediaRepository for PgMediaRepository {
    async fn create(&self, media: NewMedia) -> Result<Media, String> {
        sqlx::query_as!(
            Media,
            r#"
            INSERT INTO media (user_id, media_type, file_path)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, media_type, file_path, created_at, updated_at, deleted_at
            "#,
            media.user_id,
            media.media_type,
            media.file_path
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Media>, String> {
        sqlx::query_as!(
            Media,
            r#"SELECT id, user_id, media_type, file_path, created_at, updated_at, deleted_at FROM media WHERE id = $1 AND deleted_at IS NULL"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Media>, String> {
        sqlx::query_as!(
            Media,
            r#"SELECT id, user_id, media_type, file_path, created_at, updated_at, deleted_at FROM media WHERE user_id = $1 AND deleted_at IS NULL"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn search(
        &self,
        filter: &MediaSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Media>, String> {
        let allowed_fields = [
            FieldInfo {
                name: "media_type",
                field_type: FieldType::Text,
            },
            FieldInfo {
                name: "file_path",
                field_type: FieldType::Text,
            },
            FieldInfo {
                name: "created_at",
                field_type: FieldType::Timestamp,
            },
        ];

        let base_sql =
            "SELECT id, user_id, media_type, file_path, created_at, updated_at, deleted_at FROM media WHERE deleted_at IS NULL";

        let params = ListParams {
            search: filter.search.clone(),
            fields: None,
            sort_by,
            cursor,
            limit: Some(limit),
        };

        let built = if let Some(uid) = filter.user_id {
            build_query_with_seed(
                base_sql,
                &params,
                &allowed_fields,
                "created_at",
                SortDirection::Desc,
                &["media_type", "file_path"],
                &[("user_id = $1", BindValue::Uuid(uid))],
                1,
            )?
        } else {
            build_query(
                base_sql,
                &params,
                &allowed_fields,
                "created_at",
                SortDirection::Desc,
                &["media_type", "file_path"],
            )?
        };

        let mut query = sqlx::query_as::<_, Media>(&built.sql);
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
                    "media_type" => Some(encode_cursor_text(&last.media_type, last.id)),
                    "file_path" => Some(encode_cursor_text(&last.file_path, last.id)),
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

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE media
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        if ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            UPDATE media
            SET deleted_at = NOW()
            WHERE id = ANY($1) AND deleted_at IS NULL
            "#,
            ids
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
