// src/modules/auth/mod.rs
pub mod controller;
pub mod dto;
pub mod service;

use utoipa::OpenApi;

/// Registry for Auth module APIs
#[derive(OpenApi)]
#[openapi(
    paths(controller::signin, controller::create_token),
    components(schemas(
        dto::SigninRequest,
        dto::SigninResponse,
        dto::UserResponse,
        dto::CreateTokenRequest,
        dto::CreateTokenResponse
    ))
)]
pub struct AuthApi;
