// src/modules/user/mod.rs
pub mod controller;
pub mod dto;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(controller::get_me), components(schemas(dto::EmbeddedUserDto)))]
pub struct UserApi;
