// src/modules/auth/controller.rs
use super::{dto::*, service};
use crate::config::AppConfig;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

/// Signin via IdP and sync user info
#[utoipa::path(
    post,
    path = "/auth/signin",
    request_body = SigninRequest,
    responses((status = 200, body = SigninResponse), (status = 401, description = "Unauthorized")),
    tag = "Auth"
)]
pub async fn signin(
    State(db): State<DatabaseConnection>,
    State(config): State<AppConfig>,
    Json(payload): Json<SigninRequest>,
) -> Result<Json<SigninResponse>, (StatusCode, String)> {
    service::signin(&db, &config, payload.code).await.map(Json)
}

/// Create a personal access token for CLI
#[utoipa::path(
    post,
    path = "/auth/token",
    request_body = CreateTokenRequest,
    responses((status = 200, body = CreateTokenResponse)),
    tag = "Auth"
)]
pub async fn create_token(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, (StatusCode, String)> {
    let mock_user_id = uuid::Uuid::nil();
    service::create_token(&db, mock_user_id, payload.name)
        .await
        .map(Json)
}
