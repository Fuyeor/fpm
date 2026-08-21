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
        controller::get_metadata_parts,
        controller::search
    ),
    components(schemas(
        dto::AcquireUploadRequest,
        dto::AcquireUploadResponse,
        dto::CommitUploadRequest,
        dto::PackageSearchQuery,
        dto::PackageSearchResponse,
        dto::PackageSearchObject,
        dto::PackageSearchPackage,
        dto::PackageSearchLinks,
        dto::PackageSearchScore,
        dto::PackageSearchScoreDetail
    ))
)]
pub struct PackageApi;
