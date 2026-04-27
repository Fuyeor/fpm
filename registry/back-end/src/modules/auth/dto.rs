// src/modules/auth/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload for exchanging IdP code
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SigninRequest {
    pub code: String,
}

/// User info synced from IdP
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}

/// Signin response containing web session token and user info
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SigninResponse {
    pub access_token: String,
    pub user: UserResponse,
}

/// Request payload for generating a CLI token
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenRequest {
    pub name: String, // e.g., "MacBook Pro"
}

/// Response containing the plaintext fpm token
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenResponse {
    pub token: String, // The "fpm_xxx" token
}
