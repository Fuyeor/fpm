// src/modules/organization/controller.rs
use super::{dto::*, service};
use crate::modules::auth::middleware::CurrentUser;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
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

#[utoipa::path(
    get,
    path = "organizations/{username}",
    params(("username" = String, Path, description = "Organization username")),
    responses((status = 200, body = OrganizationProfileDto)),
    tag = "Organization"
)]
/// Returns public organization metadata.
pub async fn get_organization_profile(
    State(db): State<DatabaseConnection>,
    Path(username): Path<String>,
) -> Result<Json<OrganizationProfileDto>, (StatusCode, String)> {
    service::get_public_organization(&db, &username)
        .await
        .map(Json)
}

#[utoipa::path(
    get,
    path = "organizations/{username}/members",
    params(("username" = String, Path, description = "Organization username")),
    responses((status = 200, body = [OrganizationMemberDto])),
    tag = "Organization"
)]
/// Returns public members of an organization.
pub async fn get_organization_members(
    State(db): State<DatabaseConnection>,
    Path(username): Path<String>,
) -> Result<Json<Vec<OrganizationMemberDto>>, (StatusCode, String)> {
    service::get_public_members(&db, &username).await.map(Json)
}

#[utoipa::path(
    get,
    path = "organizations/{username}/packages",
    params(("username" = String, Path, description = "Organization username")),
    responses((status = 200, body = [OrganizationPackageDto])),
    tag = "Organization"
)]
/// Returns public package summaries of an organization.
pub async fn get_organization_packages(
    State(db): State<DatabaseConnection>,
    Path(username): Path<String>,
) -> Result<Json<Vec<OrganizationPackageDto>>, (StatusCode, String)> {
    service::get_public_packages(&db, &username).await.map(Json)
}
