// src/modules/organization/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
