// src/modules/package/controller.rs
use super::{dto::*, service};
use crate::config::AppConfig;
use crate::modules::auth::middleware::CurrentUser;
use aws_sdk_s3::Client as S3Client;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/packages/acquire",
    request_body = AcquireUploadRequest,
    responses((status = 200, body = AcquireUploadResponse)),
    tag = "Package",
    security(("token" = [])) 
)]
/// Acquire a presigned URL for uploading a package
/// The client will upload the package directly to S3 using the returned URL
/// and then call /packages/commit to finalize the upload
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
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

#[utoipa::path(
    post,
    path = "/packages/commit",
    request_body = CommitUploadRequest,
    responses((status = 201, description = "Created")),
    tag = "Package"
)]
/// Commit an upload after the client has successfully uploaded the package to S3.
/// This will create the package record in the database and make it available in the registry.
pub async fn commit_upload(
    State(db): State<DatabaseConnection>,
    State(config): State<AppConfig>,
    Json(payload): Json<CommitUploadRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    service::commit_upload(&db, &config, payload)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}
