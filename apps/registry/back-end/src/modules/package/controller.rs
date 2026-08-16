// src/modules/package/controller.rs
use super::{dto::*, service};
use crate::config::AppConfig;
use crate::modules::auth::middleware::CurrentUser;
use aws_sdk_s3::Client as S3Client;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DatabaseConnection;
use serde_json::Value;

#[utoipa::path(
    post,
    path = "/packages/acquire",
    request_body = AcquireUploadRequest,
    responses((status = 200, body = AcquireUploadResponse)),
    tag = "Package",
    security(("token" = []))
)]
/// Acquire a presigned URL for uploading a package.
pub async fn acquire_upload(
    State(db): State<DatabaseConnection>,
    State(s3_client): State<S3Client>,
    State(config): State<AppConfig>,
    user: CurrentUser,
    Json(payload): Json<AcquireUploadRequest>,
) -> Result<Json<AcquireUploadResponse>, (StatusCode, String)> {
    service::acquire_upload(&db, &s3_client, &config, &user, payload)
        .await
        .map(Json)
        .map_err(|error| (StatusCode::BAD_REQUEST, error))
}

#[utoipa::path(
    post,
    path = "/packages/commit",
    request_body = CommitUploadRequest,
    responses((status = 201, description = "Created")),
    tag = "Package",
    security(("token" = []))
)]
/// Commits an uploaded package after verifying the authenticated publisher.
pub async fn commit_upload(
    State(db): State<DatabaseConnection>,
    State(s3_client): State<S3Client>,
    State(config): State<AppConfig>,
    user: CurrentUser,
    Json(payload): Json<CommitUploadRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    service::commit_upload(&db, &s3_client, &config, user.id, payload)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|error| (StatusCode::BAD_REQUEST, error))
}

#[utoipa::path(
    get,
    path = "/{package_name}",
    params(("package_name" = String, Path, description = "URL-encoded package name")),
    responses((status = 200, description = "Abbreviated package metadata")),
    tag = "Package"
)]
/// Returns npm abbreviated metadata for a percent-encoded scoped package name.
pub async fn get_metadata(
    State(db): State<DatabaseConnection>,
    Path(raw_name): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let package_name = service::decode_package_name(&raw_name)
        .map_err(|error| (StatusCode::BAD_REQUEST, error))?;
    service::get_metadata(&db, &package_name).await.map(Json)
}

#[utoipa::path(
    get,
    path = "/{scope}/{name}",
    params(
        ("scope" = String, Path, description = "Package scope"),
        ("name" = String, Path, description = "Package name")
    ),
    responses((status = 200, description = "Abbreviated package metadata")),
    tag = "Package"
)]
/// Returns npm abbreviated metadata when a client sends the scope and package as separate segments.
pub async fn get_metadata_parts(
    State(db): State<DatabaseConnection>,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let package_name = format!("{scope}/{name}");
    let package_name = service::decode_package_name(&package_name)
        .map_err(|error| (StatusCode::BAD_REQUEST, error))?;
    service::get_metadata(&db, &package_name).await.map(Json)
}
