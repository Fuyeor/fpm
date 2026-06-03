// src/modules/user/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A lightweight user representation embedded in session/profile responses, without sensitive info
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedUserDto {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}

/// organizations user belongs to, with role info
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOrganizationDto {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

/// Response schema for the "Get My Session" API, containing user info and their organizations
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MySessionResponse {
    pub user: EmbeddedUserDto,
    pub organizations: Vec<UserOrganizationDto>,
}

/// A lightweight package summary used in profile package lists
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPackageDto {
    pub id: Uuid,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub created_at: sea_orm::prelude::DateTimeWithTimeZone,
}
