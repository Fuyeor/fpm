// src/config.rs
use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,

    // Auth
    pub idp_base_url: String,
    pub idp_client_id: String,
    pub idp_client_secret: String,
    pub idp_redirect_uri: String,
    pub jwt_key: String,

    // R2 Cloud Storage
    pub r2_endpoint: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket_name: String,
    pub r2_public_url_base: String,
}

impl AppConfig {
    /// Loads configuration from environment variables, failing fast if missing
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: require_env("DATABASE_URL"),

            idp_base_url: require_env("IDP_BASE_URL"),
            idp_client_id: require_env("IDP_CLIENT_ID"),
            idp_client_secret: require_env("IDP_CLIENT_SECRET"),
            idp_redirect_uri: require_env("IDP_REDIRECT_URI"),
            jwt_key: require_env("JWT_KEY"),

            // Load R2 Config
            r2_endpoint: require_env("R2_ENDPOINT"),
            r2_access_key_id: require_env("R2_ACCESS_KEY_ID"),
            r2_secret_access_key: require_env("R2_SECRET_ACCESS_KEY"),
            r2_bucket_name: require_env("R2_BUCKET_NAME"),
            r2_public_url_base: require_env("R2_PUBLIC_URL_BASE"),
        }
    }
}

fn require_env(key: &str) -> String {
    env::var(key)
        .unwrap_or_else(|_| panic!("Fatal: Missing required environment variable: {}", key))
}
