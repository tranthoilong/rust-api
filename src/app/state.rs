use crate::domain::repositories::{
    media_repository::MediaRepository, permission_repository::PermissionRepository,
    role_repository::RoleRepository, user_repository::UserRepository,
};
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub permission_repo: Arc<dyn PermissionRepository>,
    pub media_repo: Arc<dyn MediaRepository>,
}
