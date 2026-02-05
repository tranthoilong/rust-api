use crate::domain::entities::audit_log::AuditLog;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuditLogFilter {
    pub user_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

#[async_trait]
#[allow(dead_code)]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, log: AuditLog) -> Result<(), String>;
    async fn search(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLog>, String>;
}

#[async_trait]
impl<T: AuditLogRepository + ?Sized + Send + Sync> AuditLogRepository for std::sync::Arc<T> {
    async fn create(&self, log: AuditLog) -> Result<(), String> {
        (**self).create(log).await
    }

    async fn search(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLog>, String> {
        (**self).search(filter).await
    }
}

