use crate::app::state::AppState;
use crate::infrastructure::persistence::postgres::{
    media_repo::PgMediaRepository, permission_repo::PgPermissionRepository,
    role_repo::PgRoleRepository, user_repo::PgUserRepository,
};
use crate::interface::http::handlers::auth_handler::login;
use crate::interface::http::handlers::media_handler::{get_media, get_user_media, upload_media};
use crate::interface::http::handlers::permission_handler::{
    create_permission, delete_permission, get_permission, get_permissions, update_permission,
};
use crate::interface::http::handlers::role_handler::{
    assign_permission, create_role, delete_role, get_role, get_role_permissions, get_roles,
    revoke_permission, update_role,
};
use crate::interface::http::handlers::user_handler::{
    assign_role, create_user, delete_user, get_user, get_users, revoke_role, update_user,
};
use crate::interface::http::middleware::auth::auth_middleware;
use axum::{
    Router, middleware,
    routing::{get, post},
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

    let media_repo = Arc::new(PgMediaRepository::new(pool))
        as Arc<dyn crate::domain::repositories::media_repository::MediaRepository>;

    let state = Arc::new(AppState {
        user_repo,
        role_repo,
        permission_repo,
        media_repo,
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
                .route("/media/:id", get(get_media))
                .route("/users/:user_id/media", get(get_user_media))
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
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
