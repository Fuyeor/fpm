// src/modules/package/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

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

/// Query parameters accepted by the public package search endpoint.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct PackageSearchQuery {
    /// Search phrase. `text` is accepted as an npm-compatible alias.
    pub q: Option<String>,
    /// Npm-compatible alias for `q`.
    pub text: Option<String>,
    /// Maximum number of results. `size` is accepted as an npm-compatible alias.
    pub limit: Option<u64>,
    /// Npm-compatible alias for `limit`.
    pub size: Option<u64>,
    /// Number of results to skip. `from` is accepted as an npm-compatible alias.
    pub offset: Option<u64>,
    /// Npm-compatible alias for `offset`.
    pub from: Option<u64>,
}

impl PackageSearchQuery {
    /// Resolves aliases and clamps pagination to a predictable public range.
    pub fn normalized(&self) -> (String, u64, u64) {
        let term = self
            .q
            .as_deref()
            .or(self.text.as_deref())
            .unwrap_or_default()
            .trim()
            .to_string();
        let limit = self.limit.or(self.size).unwrap_or(20).clamp(1, 50);
        let offset = self.offset.or(self.from).unwrap_or(0);
        (term, limit, offset)
    }
}

/// npm-compatible package search response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchResponse {
    pub objects: Vec<PackageSearchObject>,
    pub total: u64,
    pub time: String,
}

/// One npm-compatible search result with a package and ranking details.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchObject {
    pub package: PackageSearchPackage,
    pub score: PackageSearchScore,
    pub search_score: f64,
}

/// Public package fields used by package discovery clients.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub date: String,
    pub links: PackageSearchLinks,
}

/// Stable links returned for a package search result.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchLinks {
    pub npm: String,
}

/// Ranking components kept compatible with npm's search response shape.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchScore {
    pub final_score: f64,
    pub detail: PackageSearchScoreDetail,
}

/// Ranking detail for one package search result.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchScoreDetail {
    pub quality: f64,
    pub popularity: f64,
    pub maintenance: f64,
}
