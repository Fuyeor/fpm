// src/modules/organization/controller.rs
use super::{dto::*, service};
use crate::modules::auth::middleware::CurrentUser;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "organizations/validation",
    request_body = CheckScopeRequest,
    responses((status = 200, body = ScopeValidationResponse)),
    tag = "Organization"
)]
/// Validate if a Scope (Organization) username is available
pub async fn validate_scope(
    State(db): State<DatabaseConnection>,
    user: Option<CurrentUser>,
    Json(payload): Json<CheckScopeRequest>,
) -> Result<Json<ScopeValidationResponse>, (StatusCode, String)> {
    let uid = user.map(|u| u.id);
    service::check_scope_availability(&db, &payload.username, uid)
        .await
        .map(Json)
        .map_err(|s| (s, "Database error".into()))
}

#[utoipa::path(
    post,
    path = "organizations",
    request_body = CreateScopeRequest,
    responses((status = 201, body = CreateScopeResponse)),
    tag = "Organization",
    security(("token" = []))
)]
/// Create a new organization (scope) for the authenticated user
pub async fn create_organization(
    State(db): State<DatabaseConnection>,
    user: CurrentUser,
    Json(payload): Json<CreateScopeRequest>,
) -> Result<Json<CreateScopeResponse>, (StatusCode, String)> {
    service::create_scope(&db, user.id, payload.username)
        .await
        .map(Json)
}
