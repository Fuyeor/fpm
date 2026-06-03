// src/modules/organization/mod.rs
pub mod controller;
pub mod dto;
pub mod service;

use utoipa::OpenApi;

/// OpenAPI documentation registry for the Organization (Scope) module.
/// This aggregates all paths and schemas inside this module.
#[derive(OpenApi)]
#[openapi(
    paths(
        controller::validate_scope,
        controller::create_organization
    ),
    components(schemas(
        dto::CheckScopeRequest,
        dto::ScopeValidationResponse,
        dto::CreateScopeRequest,
        dto::CreateScopeResponse
    )),
    tags(
        (name = "Organization", description = "Scope and organization workspace management APIs")
    )
)]
pub struct OrganizationApi;
