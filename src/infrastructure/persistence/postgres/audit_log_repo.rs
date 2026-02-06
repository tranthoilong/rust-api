use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::audit_log::AuditLog;
use crate::domain::repositories::audit_log_repository::{AuditLogFilter, AuditLogRepository};

#[allow(dead_code)]
pub struct PgAuditLogRepository {
    pool: Pool<Postgres>,
}

#[allow(dead_code)]
impl PgAuditLogRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for PgAuditLogRepository {
    async fn create(&self, log: AuditLog) -> Result<(), String> {
        sqlx::query!(
            r#"
            INSERT INTO audit_logs
            (id, user_id, action, entity_type, entity_id,
             metadata, old_data, new_data, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            log.id,
            log.user_id,
            log.action,
            log.entity_type,
            log.entity_id,
            log.metadata,
            log.old_data,
            log.new_data,
            log.ip_address,
            log.user_agent
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn search(&self, _filter: &AuditLogFilter) -> Result<Vec<AuditLog>, String> {
        // Phiên bản đơn giản: trả về 100 bản ghi mới nhất, tránh lỗi bind động.
        let logs = sqlx::query_as!(
            AuditLog,
            r#"
            SELECT id, user_id, action, entity_type, entity_id,
                   metadata as "metadata: _",
                   old_data as "old_data: _",
                   new_data as "new_data: _",
                   ip_address, user_agent, created_at
            FROM audit_logs
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(logs)
    }
}

