// src/main.rs
mod config;
mod entities;
mod modules;
mod services;
mod utils;

use crate::modules::auth::{AuthApi, controller as auth_ctrl};
use axum::extract::FromRef;
use axum::{Router, routing::post};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: config::AppConfig,
}

// Implement FromRef so Axum can extract individual parts
impl FromRef<AppState> for sea_orm::DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for config::AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

#[derive(OpenApi)]
#[openapi(info(title = "fpm registry API", version = "0.1.0"))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::load();
    let db = sea_orm::Database::connect(&config.database_url)
        .await
        .unwrap();

    let state = AppState { db, config };

    // Create and merge OpenAPI specs
    let mut openapi = ApiDoc::openapi();
    openapi.merge(AuthApi::openapi());

    // Build Router
    let app = Router::new()
        .route("/auth/signin", post(auth_ctrl::signin))
        .route("/auth/token", post(auth_ctrl::create_token))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", openapi))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 5590));
    println!("🚀 FPM Registry is running on http://{}", addr);
    println!("📖 Swagger UI is at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
