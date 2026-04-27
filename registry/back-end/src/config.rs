// src/config.rs
use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub idp_base_url: String,
    pub idp_client_id: String,
    pub idp_client_secret: String,
    pub idp_redirect_uri: String,
    pub jwt_key: String,
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
        }
    }
}

fn require_env(key: &str) -> String {
    env::var(key)
        .unwrap_or_else(|_| panic!("Fatal: Missing required environment variable: {}", key))
}
