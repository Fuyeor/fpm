// src/modules/auth/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request payload for exchanging IdP code
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SigninRequest {
    pub code: String,
}

/// User info synced from IdP
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}

/// Signin response containing web session token and user info
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SigninResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserDto,
}

/// Request payload for generating a CLI token
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenRequest {
    pub name: String, // e.g., "MacBook Pro"
}
/// Response containing the plaintext fpm token
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenResponse {
    pub token: String, // The "fpm_xxx" token
}
