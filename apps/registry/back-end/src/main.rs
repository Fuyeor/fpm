// src/main.rs
mod config;
mod entities;
mod modules;
mod services;
mod utils;

use aws_sdk_s3::Client as S3Client;
use axum::{Router, extract::FromRef, routing::get, routing::post};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::modules::auth::{AuthApi, controller as auth};
use crate::modules::organization::{OrganizationApi, controller as organization};
use crate::modules::package::{PackageApi, controller as package};
use crate::modules::user::{UserApi, controller as user};

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: config::AppConfig,
    pub s3: S3Client,
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

impl FromRef<AppState> for S3Client {
    fn from_ref(state: &AppState) -> Self {
        state.s3.clone()
    }
}

#[derive(OpenApi)]
#[openapi(info(title = "fpm.fuyeor.com API"))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::load();
    let db = sea_orm::Database::connect(&config.database_url)
        .await
        .unwrap();
    let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest()) // 使用最新的行为版本
        .endpoint_url(&config.r2_endpoint)
        .region(aws_sdk_s3::config::Region::new("auto"))
        .load()
        .await;
    let s3_client = aws_sdk_s3::Client::new(&s3_config);

    let state = AppState {
        db,
        config,
        s3: s3_client,
    };

    // Create and merge OpenAPI specs
    let mut openapi = ApiDoc::openapi();
    openapi.merge(AuthApi::openapi());
    openapi.merge(PackageApi::openapi());
    openapi.merge(UserApi::openapi());
    openapi.merge(OrganizationApi::openapi());

    // Build Router
    let app = Router::new()
        // Auth Routes
        .route("/auth/signin", post(auth::signin))
        .route("/auth/token", post(auth::create_token))
        .route("/auth/refresh-token", post(auth::refresh))
        // User Routes
        .route("/users/me", get(user::get_me))
        .route("/users/:username", get(user::get_user_profile))
        .route(
            "/users/:username/organizations",
            get(user::get_user_organizations),
        )
        .route("/users/:username/packages", get(user::get_user_packages))
        // organization Routes
        .route(
            "/organizations/validation",
            post(organization::validate_scope),
        )
        .route("/organizations", post(organization::create_organization))
        // Package Routes
        .route("/packages/acquire", post(package::acquire_upload))
        .route("/packages/commit", post(package::commit_upload))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", openapi))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 6011));
    println!("🚀 FPM Registry is running on http://{}", addr);
    println!("📖 Swagger UI is at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // allow axum to extract ip address in handlers
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
