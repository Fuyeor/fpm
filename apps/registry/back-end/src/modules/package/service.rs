// src/modules/package/service.rs
use super::dto::{AcquireUploadRequest, AcquireUploadResponse, CommitUploadRequest};
use crate::{
    config::AppConfig,
    entities::{organization, organization_member, package, package_version, prelude::*},
    modules::auth::middleware::CurrentUser,
};
use aws_sdk_s3::{Client as S3Client, presigning::PresigningConfig};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const UPLOAD_SESSION_EXP_SECONDS: u64 = 900; // 15 minutes

#[derive(serde::Serialize, serde::Deserialize)]
struct UploadSession {
    pub exp: usize, // Required for JWT
    pub user_id: Uuid,
    pub package_name: String,
    pub package_version: String,
    pub shasum: String,
}

/// Generates a presigned URL and a JWT upload session.
pub async fn acquire_upload(
    db: &DatabaseConnection,
    s3_client: &S3Client,
    config: &AppConfig,
    user: &CurrentUser,
    req: AcquireUploadRequest,
) -> Result<AcquireUploadResponse, String> {
    // 1. Validate package name format
    let parts: Vec<&str> = req.name.split('/').collect();
    if parts.len() != 2 || !parts[0].starts_with('@') {
        return Err("Invalid package name format. Must be @scope/package.".into());
    }
    let scope_name = &parts[0][1..];

    // 2. Check permissions
    let org = Organization::find()
        .filter(organization::Column::Name.eq(scope_name))
        .one(db)
        .await
        .unwrap()
        .ok_or_else(|| "Organization scope not found.".to_string())?;

    let is_member = OrganizationMember::find()
        .filter(organization_member::Column::OrganizationId.eq(org.id))
        .filter(organization_member::Column::UserId.eq(user.id))
        .one(db)
        .await
        .unwrap()
        .is_some();

    if !is_member {
        return Err("You do not have permission to publish to this scope.".into());
    }

    // 3. Check for existing version
    let existing_version = PackageVersion::find()
        .inner_join(Package)
        .filter(package::Column::FullName.eq(&req.name))
        .filter(package_version::Column::Version.eq(&req.version))
        .one(db)
        .await
        .unwrap();

    if existing_version.is_some() {
        return Err("This package version already exists.".into());
    }

    // 4. Create JWT upload session
    let exp = (SystemTime::now() + Duration::from_secs(UPLOAD_SESSION_EXP_SECONDS))
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let session = UploadSession {
        exp,
        user_id: user.id,
        package_name: req.name,
        package_version: req.version,
        shasum: req.shasum,
    };
    let jwt_key = EncodingKey::from_secret(config.jwt_key.as_bytes());
    let upload_session_id = encode(&Header::default(), &session, &jwt_key).unwrap();

    // 5. Generate R2 Presigned URL
    let object_key = format!("{}/{}", session.package_name, session.package_version);
    let presigned_request = s3_client
        .put_object()
        .bucket(&config.r2_bucket_name)
        .key(object_key)
        .presigned(
            PresigningConfig::expires_in(Duration::from_secs(UPLOAD_SESSION_EXP_SECONDS)).unwrap(),
        )
        .await
        .unwrap();

    Ok(AcquireUploadResponse {
        upload_url: presigned_request.uri().to_string(),
        upload_session_id,
    })
}

/// Commits the package metadata to the database.
pub async fn commit_upload(
    db: &DatabaseConnection,
    config: &AppConfig,
    req: CommitUploadRequest,
) -> Result<(), String> {
    // 1. Decode and validate JWT
    let jwt_key = DecodingKey::from_secret(config.jwt_key.as_bytes());
    let session: UploadSession = decode(&req.upload_session_id, &jwt_key, &Validation::default())
        .unwrap()
        .claims;

    // 2. Find or create the package record
    let pkg = match Package::find()
        .filter(package::Column::FullName.eq(&session.package_name))
        .one(db)
        .await
        .unwrap()
    {
        Some(p) => p,
        None => {
            let scope_name = &session.package_name.split('/').collect::<Vec<&str>>()[0][1..];
            let org = Organization::find()
                .filter(organization::Column::Name.eq(scope_name))
                .one(db)
                .await
                .unwrap()
                .unwrap();

            let new_pkg = package::ActiveModel {
                id: Set(Uuid::now_v7()),
                organization_id: Set(org.id),
                name: Set(session.package_name.split('/').last().unwrap().to_string()),
                full_name: Set(session.package_name.clone()),
                description: Set(req
                    .manifest
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())),
                ..Default::default()
            };
            new_pkg.insert(db).await.unwrap()
        }
    };

    // 3. Create the package version record
    let new_version = package_version::ActiveModel {
        id: Set(Uuid::now_v7()),
        package_id: Set(pkg.id),
        version: Set(session.package_version.clone()),
        manifest: Set(req.manifest),
        dist_tarball: Set(format!(
            "{}/{}/{}",
            config.r2_public_url_base, session.package_name, session.package_version
        )),
        dist_shasum: Set(session.shasum),
        ..Default::default()
    };
    new_version.insert(db).await.unwrap();

    Ok(())
}
