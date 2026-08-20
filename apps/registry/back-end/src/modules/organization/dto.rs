// src/modules/organization/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Public organization profile metadata.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationProfileDto {
    pub id: Uuid,
    pub username: String,
    pub description: Option<String>,
    pub created_at: sea_orm::prelude::DateTimeWithTimeZone,
}

/// Public organization member with the user's role in the organization.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationMemberDto {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub role: String,
    pub created_at: sea_orm::prelude::DateTimeWithTimeZone,
}

/// Public package summary belonging to an organization.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationPackageDto {
    pub id: Uuid,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub created_at: sea_orm::prelude::DateTimeWithTimeZone,
}

/// Request payload to check if a Scope name is available
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckScopeRequest {
    pub username: String,
}

/// Response payload for Scope validation
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScopeValidationResponse {
    pub available: bool,
    pub message: String,
}

/// Request payload to create a new Scope (Organization)
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateScopeRequest {
    pub username: String,
}

/// Response payload after a successful Scope creation
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateScopeResponse {
    pub id: Uuid,
    pub username: String,
}
