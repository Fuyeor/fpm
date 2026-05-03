// src/modules/package/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request to acquire a presigned URL for upload.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcquireUploadRequest {
    /// Full package name, e.g., "@fuyeor/fpm-cli".
    pub name: String,
    /// Package version, e.g., "0.1.0".
    pub version: String,
    /// SHA-256 hash of the tarball to be uploaded.
    pub shasum: String,
}

/// Response containing the presigned URL.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcquireUploadResponse {
    /// The temporary URL for the client to PUT the file.
    pub upload_url: String,
    /// A session ID to be used in the commit phase.
    pub upload_session_id: String,
}

/// Request to commit the upload after the file is in R2.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommitUploadRequest {
    /// The session ID from the acquire phase.
    pub upload_session_id: String,
    /// The full manifest (package.json) content.
    pub manifest: serde_json::Value,
}
