use std::sync::Arc;

use crate::domain::entities::audit_log::AuditLog;
use crate::domain::repositories::audit_log_repository::{AuditLogFilter, AuditLogRepository};

pub struct GetAuditLogsUseCase {
    repo: Arc<dyn AuditLogRepository>,
}

impl GetAuditLogsUseCase {
    pub fn new(repo: Arc<dyn AuditLogRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, filter: AuditLogFilter) -> Result<Vec<AuditLog>, String> {
        self.repo
            .search(&filter)
            .await
            .map_err(|e| format!("Failed to search audit logs: {}", e))
    }
}

