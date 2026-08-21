// src/main.rs
mod api;
mod config;
mod entities;
mod health;
mod modules;
mod services;
mod utils;

use aws_sdk_s3::Client as S3Client;
use axum::{Router, extract::FromRef, routing::delete, routing::get, routing::post};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::sitemap::SitemapApi;
use crate::api::{package as public_package, search as public_search, sitemap as public_sitemap};
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
    // create Credentials
    let credentials = aws_sdk_s3::config::Credentials::new(
        &config.r2_access_key_id,
        &config.r2_secret_access_key,
        None,
        None,
        "static",
    );

    // inject Credentials into S3 configs
    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version_latest()
        .endpoint_url(&config.r2_endpoint)
        .region(aws_sdk_s3::config::Region::new("auto"))
        .credentials_provider(credentials)
        .build();

    let s3_client = aws_sdk_s3::Client::from_conf(s3_config);

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
    openapi.merge(SitemapApi::openapi());

    // Build Router
    let app = Router::new()
        // Auth Routes
        .route("/auth/signin", post(auth::signin))
        .route("/auth/refresh-token", post(auth::refresh))
        .route("/auth/token", post(auth::create_token))
        .route("/auth/tokens", get(auth::list_tokens))
        .route("/auth/tokens/:id", delete(auth::revoke_token))
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
        .route(
            "/organizations/:username",
            get(organization::get_organization_profile),
        )
        .route(
            "/organizations/:username/members",
            get(organization::get_organization_members),
        )
        .route(
            "/organizations/:username/packages",
            get(organization::get_organization_packages),
        )
        // Package publishing routes
        .route("/packages/acquire", post(package::acquire_upload))
        .route("/packages/commit", post(package::commit_upload))
        // Public sitemap XML routes
        .route("/sitemaps/index.xml", get(public_sitemap::get_index))
        .route("/sitemaps/en/users.xml", get(public_sitemap::get_users))
        .route(
            "/sitemaps/en/organizations.xml",
            get(public_sitemap::get_organizations),
        )
        .route(
            "/sitemaps/en/packages.xml",
            get(public_sitemap::get_packages),
        )
        // Public package discovery and npm-compatible metadata routes
        .route("/search", get(public_search::search))
        .route("/:package_name", get(public_package::get_metadata))
        .route("/:scope/:name", get(public_package::get_metadata_parts))
        // Process liveness probe; nginx exposes this as /v1/health
        .route("/health", get(health::health))
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
