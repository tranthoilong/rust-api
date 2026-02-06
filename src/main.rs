use crate::app::state::AppState;
use crate::infrastructure::persistence::postgres::{
    audit_log_repo::PgAuditLogRepository, banner_repo::PgBannerRepository,
    category_repo::PgCategoryRepository, language_repo::PgLanguageRepository,
    media_repo::PgMediaRepository, permission_repo::PgPermissionRepository,
    post_repo::PgPostRepository, role_repo::PgRoleRepository, setting_repo::PgSettingRepository,
    tag_repo::PgTagRepository, user_profile_repo::PgUserProfileRepository,
    user_repo::PgUserRepository,
};
use crate::interface::http::handlers::audit_log_handler::list_audit_logs;
use crate::interface::http::handlers::auth_handler::{login, me, register};
use crate::interface::http::handlers::banner_handler::{
    bulk_delete_banners, create_banner, create_banner_item, delete_banner, delete_banner_item,
    get_banner_by_key, list_active_banners, update_banner, update_banner_item,
};
use crate::interface::http::handlers::category_handler::{
    bulk_delete_categories, create_category, delete_category, get_category, list_categories,
    update_category,
};
use crate::interface::http::handlers::language_handler::{get_default_language, list_languages};
use crate::interface::http::handlers::media_handler::{
    bulk_delete_media, delete_media, get_media, get_user_media, update_media, upload_media,
};
use crate::interface::http::handlers::permission_handler::{
    create_permission, delete_permission, get_permission, get_permissions, update_permission,
};
use crate::interface::http::handlers::post_handler::{
    bulk_delete_posts, create_post, delete_post, get_post_by_slug, list_posts, update_post,
};
use crate::interface::http::handlers::role_handler::{
    assign_permission, create_role, delete_role, get_role, get_role_permissions, get_roles,
    revoke_permission, update_role,
};
use crate::interface::http::handlers::settings_handler::{
    get_setting as get_setting_handler, list_settings, update_setting as update_setting_handler,
};
use crate::interface::http::handlers::tag_handler::{
    bulk_delete_tags, create_tag, delete_tag, get_tag, list_tags, update_tag,
};
use crate::interface::http::handlers::user_handler::{
    assign_role, create_user, delete_user, get_user, get_users, revoke_role, update_user,
};
use crate::interface::http::handlers::user_profile_handler::{get_me_profile, update_me_profile};
use crate::interface::http::middleware::auth::auth_middleware;
use axum::{
    Router, middleware,
    routing::{get, patch, post},
};
use std::sync::Arc;

mod app;
mod application;
mod domain;
mod infrastructure;
mod interface;
mod shared;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let prefix_api = "/api/v1";

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let user_repo = Arc::new(PgUserRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::user_repository::UserRepository>;

    let role_repo = Arc::new(PgRoleRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::role_repository::RoleRepository>;

    let permission_repo = Arc::new(PgPermissionRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::permission_repository::PermissionRepository>;

    let media_repo = Arc::new(PgMediaRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::media_repository::MediaRepository>;

    let setting_repo = Arc::new(PgSettingRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::setting_repository::SettingRepository>;

    let banner_repo = Arc::new(PgBannerRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::banner_repository::BannerRepository>;

    let user_profile_repo = Arc::new(PgUserProfileRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::user_profile_repository::UserProfileRepository>;

    let category_repo = Arc::new(PgCategoryRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::category_repository::CategoryRepository>;

    let tag_repo = Arc::new(PgTagRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::tag_repository::TagRepository>;

    let post_repo = Arc::new(PgPostRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::post_repository::PostRepository>;

    let language_repo = Arc::new(PgLanguageRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::language_repository::LanguageRepository>;

    let audit_log_repo = Arc::new(PgAuditLogRepository::new(pool.clone()))
        as Arc<dyn crate::domain::repositories::audit_log_repository::AuditLogRepository>;

    let state = Arc::new(AppState {
        user_repo,
        role_repo,
        permission_repo,
        media_repo,
        setting_repo,
        banner_repo,
        user_profile_repo,
        category_repo,
        tag_repo,
        post_repo,
        language_repo,
        audit_log_repo,
    });

    let app = Router::new()
        .route("/", get(|| async { "Rust API is running!" }))
        .nest(
            prefix_api,
            Router::new()
                .route("/users", get(get_users).post(create_user))
                .route(
                    "/users/:id",
                    get(get_user).patch(update_user).delete(delete_user),
                )
                .route(
                    "/users/:user_id/roles/:role_id",
                    post(assign_role).delete(revoke_role),
                )
                .route("/roles", get(get_roles).post(create_role))
                .route(
                    "/roles/:id",
                    get(get_role).patch(update_role).delete(delete_role),
                )
                .route("/roles/:role_id/permissions", get(get_role_permissions))
                .route(
                    "/roles/:role_id/permissions/:permission_id",
                    post(assign_permission).delete(revoke_permission),
                )
                .route("/permissions", get(get_permissions).post(create_permission))
                .route(
                    "/permissions/:id",
                    get(get_permission)
                        .patch(update_permission)
                        .delete(delete_permission),
                )
                .route("/media", post(upload_media))
                .route(
                    "/media/:id",
                    get(get_media).patch(update_media).delete(delete_media),
                )
                .route("/media/bulk-delete", post(bulk_delete_media))
                .route("/users/:user_id/media", get(get_user_media))
                .route("/auth/me", get(me))
                .route("/me/profile", get(get_me_profile).put(update_me_profile))
                // Settings
                .route("/settings", get(list_settings))
                .route(
                    "/settings/:key",
                    get(get_setting_handler).put(update_setting_handler),
                )
                // Banners
                .route("/banners/active", get(list_active_banners))
                .route("/banners", post(create_banner))
                .route("/banners/key/:key", get(get_banner_by_key))
                .route("/banners/:id", patch(update_banner).delete(delete_banner))
                .route("/banners/bulk-delete", post(bulk_delete_banners))
                // Banner Items
                .route("/banner-items", post(create_banner_item))
                .route(
                    "/banner-items/:id",
                    patch(update_banner_item).delete(delete_banner_item),
                )
                // Categories & Tags & Posts (blog / content)
                .route("/categories", get(list_categories).post(create_category))
                .route(
                    "/categories/:slug",
                    get(get_category)
                        .patch(update_category)
                        .delete(delete_category),
                )
                .route("/categories/bulk-delete", post(bulk_delete_categories))
                .route("/tags", get(list_tags).post(create_tag))
                .route(
                    "/tags/:slug",
                    get(get_tag).patch(update_tag).delete(delete_tag),
                )
                .route("/tags/bulk-delete", post(bulk_delete_tags))
                .route("/posts", get(list_posts).post(create_post))
                .route(
                    "/posts/:slug",
                    get(get_post_by_slug).patch(update_post).delete(delete_post),
                )
                .route("/posts/bulk-delete", post(bulk_delete_posts))
                // Languages
                .route("/languages", get(list_languages))
                .route("/languages/default", get(get_default_language))
                // Audit logs (admin area)
                .route("/audit-logs", get(list_audit_logs))
                .nest(
                    "/admin",
                    Router::new()
                        .route("/dashboard", get(|| async { "Admin Dashboard" }))
                        .route_layer(middleware::from_fn_with_state(
                            state.clone(),
                            crate::interface::http::middleware::permission::require_admin_role,
                        )),
                )
                .route_layer(middleware::from_fn(auth_middleware)),
        )
        .route(&format!("{}/auth/login", prefix_api), post(login))
        .route(&format!("{}/auth/register", prefix_api), post(register))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
