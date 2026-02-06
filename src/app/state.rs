use crate::domain::repositories::{
    audit_log_repository::AuditLogRepository, banner_repository::BannerRepository,
    category_repository::CategoryRepository, language_repository::LanguageRepository,
    media_repository::MediaRepository, permission_repository::PermissionRepository,
    post_repository::PostRepository, role_repository::RoleRepository,
    setting_repository::SettingRepository, tag_repository::TagRepository,
    user_profile_repository::UserProfileRepository, user_repository::UserRepository,
};
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub permission_repo: Arc<dyn PermissionRepository>,
    pub media_repo: Arc<dyn MediaRepository>,
    pub setting_repo: Arc<dyn SettingRepository>,
    pub banner_repo: Arc<dyn BannerRepository>,
    pub user_profile_repo: Arc<dyn UserProfileRepository>,
    pub category_repo: Arc<dyn CategoryRepository>,
    pub tag_repo: Arc<dyn TagRepository>,
    pub post_repo: Arc<dyn PostRepository>,
    pub language_repo: Arc<dyn LanguageRepository>,
    pub audit_log_repo: Arc<dyn AuditLogRepository>,
}
