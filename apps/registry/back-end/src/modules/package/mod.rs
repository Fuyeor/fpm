// src/modules/package/mod.rs
pub mod controller;
pub mod dto;
pub mod service;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::acquire_upload,
        controller::commit_upload,
        controller::get_metadata,
        controller::get_metadata_parts
    ),
    components(schemas(
        dto::AcquireUploadRequest,
        dto::AcquireUploadResponse,
        dto::CommitUploadRequest
    ))
)]
pub struct PackageApi;
