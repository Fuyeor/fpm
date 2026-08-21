// src/modules/package/service.rs
use super::dto::{
    AcquireUploadRequest, AcquireUploadResponse, CommitUploadRequest, PackageSearchLinks,
    PackageSearchObject, PackageSearchPackage, PackageSearchQuery, PackageSearchResponse,
    PackageSearchScore, PackageSearchScoreDetail,
};
use crate::{
    config::AppConfig,
    entities::{organization, organization_member, package, package_version, prelude::*},
    modules::auth::middleware::CurrentUser,
};
use aws_sdk_s3::{Client as S3Client, presigning::PresigningConfig};
use axum::http::StatusCode;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use percent_encoding::{AsciiSet, CONTROLS, percent_decode_str, utf8_percent_encode};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use semver::Version;
use serde_json::{Map, Value, json};
use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const UPLOAD_SESSION_EXP_SECONDS: u64 = 900;
const URL_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`');

#[derive(serde::Serialize, serde::Deserialize)]
struct UploadSession {
    pub exp: usize,
    pub user_id: Uuid,
    pub package_name: String,
    pub package_version: String,
    pub shasum: String,
}

/// Validates the scoped package name and returns its scope and local name.
pub fn validate_package_name(name: &str) -> Result<(&str, &str), String> {
    let slash = name
        .find('/')
        .ok_or_else(|| "Invalid package name. Expected @scope/package.".to_string())?;

    let scope = &name[1..slash];
    let package = &name[slash + 1..];
    if !name.starts_with('@')
        || scope.is_empty()
        || package.is_empty()
        || package.contains('/')
        || scope == "."
        || scope == ".."
        || package == "."
        || package == ".."
        || name.contains('\\')
        || name.chars().any(char::is_control)
    {
        return Err("Invalid package name. Expected @scope/package.".to_string());
    }

    Ok((scope, package))
}

/// Decodes a package name from either an encoded or an already decoded URL path.
pub fn decode_package_name(raw_name: &str) -> Result<String, String> {
    let decoded = percent_decode_str(raw_name)
        .decode_utf8()
        .map_err(|_| "Package name contains invalid URL encoding.".to_string())?
        .into_owned();
    validate_package_name(&decoded)?;
    Ok(decoded)
}

/// Builds the public object key shared by R2 and package metadata.
pub fn object_key(package_name: &str, package_version: &str) -> String {
    format!("packages/{package_name}/{package_version}.tgz")
}

/// Builds a stable public tarball URL without relying on request host headers.
pub fn public_tarball_url(config: &AppConfig, package_name: &str, package_version: &str) -> String {
    let (scope, name) = validate_package_name(package_name).expect("validated package name");
    let scope_segment = format!("@{scope}");
    let encoded_scope = utf8_percent_encode(&scope_segment, URL_SEGMENT_ENCODE_SET).to_string();
    let encoded_name = utf8_percent_encode(name, URL_SEGMENT_ENCODE_SET).to_string();
    let encoded_version = utf8_percent_encode(package_version, URL_SEGMENT_ENCODE_SET).to_string();
    format!(
        "{}/packages/{}/{}/{}.tgz",
        config.r2_public_url_base.trim_end_matches('/'),
        encoded_scope,
        encoded_name,
        encoded_version
    )
}

/// Generates a presigned URL and a short-lived upload session.
pub async fn acquire_upload(
    db: &DatabaseConnection,
    s3_client: &S3Client,
    config: &AppConfig,
    user: &CurrentUser,
    req: AcquireUploadRequest,
) -> Result<AcquireUploadResponse, String> {
    let (scope_name, _) = validate_package_name(&req.name)?;
    Version::parse(&req.version)
        .map_err(|_| "Package version must be valid SemVer.".to_string())?;
    if hex::decode(&req.shasum)
        .map(|bytes| bytes.len() == 32)
        .unwrap_or(false)
        == false
    {
        return Err("Package shasum must be a 64-character SHA-256 hex digest.".to_string());
    }

    let org = Organization::find()
        .filter(organization::Column::Username.eq(scope_name))
        .one(db)
        .await
        .map_err(|_| "Failed to query package scope.".to_string())?
        .ok_or_else(|| "Organization scope not found.".to_string())?;

    let is_member = OrganizationMember::find()
        .filter(organization_member::Column::OrganizationId.eq(org.id))
        .filter(organization_member::Column::UserId.eq(user.id))
        .one(db)
        .await
        .map_err(|_| "Failed to query scope membership.".to_string())?
        .is_some();

    if !is_member {
        return Err("You do not have permission to publish to this scope.".to_string());
    }

    let existing_version = PackageVersion::find()
        .inner_join(Package)
        .filter(package::Column::FullName.eq(&req.name))
        .filter(package_version::Column::Version.eq(&req.version))
        .one(db)
        .await
        .map_err(|_| "Failed to query existing package version.".to_string())?;

    if existing_version.is_some() {
        return Err("This package version already exists.".to_string());
    }

    let exp = (SystemTime::now() + Duration::from_secs(UPLOAD_SESSION_EXP_SECONDS))
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System clock is before Unix epoch.".to_string())?
        .as_secs() as usize;
    let session = UploadSession {
        exp,
        user_id: user.id,
        package_name: req.name,
        package_version: req.version,
        shasum: req.shasum,
    };
    let jwt_key = EncodingKey::from_secret(config.jwt_key.as_bytes());
    let upload_session_id = encode(&Header::default(), &session, &jwt_key)
        .map_err(|_| "Failed to create upload session.".to_string())?;

    let presigned_request = s3_client
        .put_object()
        .bucket(&config.r2_bucket_name)
        .key(object_key(&session.package_name, &session.package_version))
        .presigned(
            PresigningConfig::expires_in(Duration::from_secs(UPLOAD_SESSION_EXP_SECONDS))
                .map_err(|_| "Invalid upload session expiry.".to_string())?,
        )
        .await
        .map_err(|_| "Failed to create upload URL.".to_string())?;

    Ok(AcquireUploadResponse {
        upload_url: presigned_request.uri().to_string(),
        upload_session_id,
    })
}

/// Commits a package only when the authenticated publisher owns the upload session.
pub async fn commit_upload(
    db: &DatabaseConnection,
    s3_client: &S3Client,
    config: &AppConfig,
    user_id: Uuid,
    req: CommitUploadRequest,
) -> Result<(), String> {
    let jwt_key = DecodingKey::from_secret(config.jwt_key.as_bytes());
    let session = decode::<UploadSession>(&req.upload_session_id, &jwt_key, &Validation::default())
        .map_err(|_| "Invalid or expired upload session.".to_string())?
        .claims;

    if session.user_id != user_id {
        return Err("Upload session does not belong to the authenticated user.".to_string());
    }

    let manifest_name = req
        .manifest
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "Manifest must contain a name.".to_string())?;
    let manifest_version = req
        .manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| "Manifest must contain a version.".to_string())?;

    if manifest_name != session.package_name || manifest_version != session.package_version {
        return Err("Manifest name/version does not match the upload session.".to_string());
    }

    s3_client
        .head_object()
        .bucket(&config.r2_bucket_name)
        .key(object_key(&session.package_name, &session.package_version))
        .send()
        .await
        .map_err(|_| "Uploaded package tarball was not found in object storage.".to_string())?;

    let (scope_name, package_name) = validate_package_name(&session.package_name)?;
    let pkg = match Package::find()
        .filter(package::Column::FullName.eq(&session.package_name))
        .one(db)
        .await
        .map_err(|_| "Failed to query package metadata.".to_string())?
    {
        Some(package) => package,
        None => {
            let org = Organization::find()
                .filter(organization::Column::Username.eq(scope_name))
                .one(db)
                .await
                .map_err(|_| "Failed to query package scope.".to_string())?
                .ok_or_else(|| "Organization scope not found.".to_string())?;

            package::ActiveModel {
                id: Set(Uuid::now_v7()),
                organization_id: Set(org.id),
                name: Set(package_name.to_string()),
                full_name: Set(session.package_name.clone()),
                description: Set(req
                    .manifest
                    .get("description")
                    .and_then(Value::as_str)
                    .map(str::to_string)),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|_| "Failed to create package metadata.".to_string())?
        }
    };

    let new_version = package_version::ActiveModel {
        id: Set(Uuid::now_v7()),
        package_id: Set(pkg.id),
        version: Set(session.package_version.clone()),
        manifest: Set(req.manifest),
        dist_tarball: Set(public_tarball_url(
            config,
            &session.package_name,
            &session.package_version,
        )),
        dist_shasum: Set(session.shasum),
        ..Default::default()
    };
    new_version
        .insert(db)
        .await
        .map_err(|_| "Package version already exists or could not be stored.".to_string())?;

    Ok(())
}

/// Returns npm's abbreviated metadata shape with only install-relevant fields.
pub async fn get_metadata(
    db: &DatabaseConnection,
    package_name: &str,
) -> Result<Value, (StatusCode, String)> {
    let package = Package::find()
        .filter(package::Column::FullName.eq(package_name))
        .one(db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to query package.".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Package not found.".to_string()))?;

    let versions = PackageVersion::find()
        .filter(package_version::Column::PackageId.eq(package.id))
        .all(db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to query package versions.".to_string(),
            )
        })?;

    if versions.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            "Package has no published versions.".to_string(),
        ));
    }

    let mut latest_version: Option<(Version, &package_version::Model)> = None;
    let mut version_values = Map::new();
    for version in &versions {
        if let Ok(parsed) = Version::parse(&version.version)
            && latest_version
                .as_ref()
                .is_none_or(|(latest, _)| parsed > *latest)
        {
            latest_version = Some((parsed, version));
        }
        version_values.insert(
            version.version.clone(),
            abbreviated_version(&package.full_name, version),
        );
    }

    let latest = latest_version
        .map(|(_, version)| version.version.clone())
        .or_else(|| versions.iter().map(|version| version.version.clone()).max())
        .ok_or((
            StatusCode::NOT_FOUND,
            "Package has no valid versions.".to_string(),
        ))?;
    let modified = versions
        .iter()
        .map(|version| version.created_at)
        .max()
        .map(|time| time.to_rfc3339())
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    Ok(json!({
        "name": package.full_name,
        "modified": modified,
        "dist-tags": { "latest": latest },
        "versions": version_values,
    }))
}

/// Creates one abbreviated metadata version object while preserving manifest dependency fields.
fn abbreviated_version(package_name: &str, version: &package_version::Model) -> Value {
    let source = match version.manifest.clone() {
        Value::Object(object) => object,
        _ => Map::new(),
    };
    let mut manifest = Map::new();
    manifest.insert("name".to_string(), Value::String(package_name.to_string()));
    manifest.insert(
        "version".to_string(),
        Value::String(version.version.clone()),
    );

    // Keep only npm abbreviated metadata fields needed by pnpm resolution and install.
    for field in [
        "dependencies",
        "optionalDependencies",
        "peerDependencies",
        "peerDependenciesMeta",
        "bin",
        "engines",
        "os",
        "cpu",
        "deprecated",
        "bundleDependencies",
        "acceptDependencies",
        "directories",
        "funding",
        "hasInstallScript",
    ] {
        if let Some(value) = source.get(field) {
            manifest.insert(field.to_string(), value.clone());
        }
    }

    let mut dist = Map::new();
    dist.insert(
        "tarball".to_string(),
        Value::String(version.dist_tarball.clone()),
    );
    dist.insert(
        "shasum".to_string(),
        Value::String(version.dist_shasum.clone()),
    );
    if let Ok(bytes) = hex::decode(&version.dist_shasum) {
        dist.insert(
            "integrity".to_string(),
            Value::String(format!("sha256-{}", BASE64_STANDARD.encode(bytes))),
        );
    }
    manifest.insert("dist".to_string(), Value::Object(dist));
    Value::Object(manifest)
}

#[cfg(test)]
mod tests {
    use super::{
        PackageSearchQuery, decode_package_name, object_key, package_search_score,
        validate_package_name,
    };

    #[test]
    fn accepts_scoped_package_names() {
        assert_eq!(
            validate_package_name("@fuyeor/commons").unwrap(),
            ("fuyeor", "commons")
        );
        assert_eq!(
            validate_package_name("@org-with-dash/tool").unwrap(),
            ("org-with-dash", "tool")
        );
    }

    #[test]
    fn rejects_unscoped_or_nested_package_names() {
        assert!(validate_package_name("commons").is_err());
        assert!(validate_package_name("@scope/").is_err());
        assert!(validate_package_name("@scope/name/extra").is_err());
    }

    #[test]
    fn decodes_metadata_path_and_builds_public_object_key() {
        assert_eq!(
            decode_package_name("%40fuyeor%2Fcommons").unwrap(),
            "@fuyeor/commons"
        );
        assert_eq!(
            object_key("@fuyeor/commons", "1.0.0"),
            "packages/@fuyeor/commons/1.0.0.tgz"
        );
    }

    #[test]
    fn normalizes_search_aliases_and_bounds_limit() {
        let query = PackageSearchQuery {
            q: None,
            text: Some(" vue ".to_string()),
            limit: Some(100),
            size: None,
            offset: None,
            from: Some(4),
        };
        assert_eq!(query.normalized(), ("vue".to_string(), 50, 4));
    }

    #[test]
    fn ranks_package_name_before_description_match() {
        assert_eq!(
            package_search_score("@fuyeor/vue", None, "@fuyeor/vue"),
            1.0
        );
        assert_eq!(package_search_score("@fuyeor/vue", None, "vue"), 0.8);
        assert_eq!(
            package_search_score("@fuyeor/tool", Some("Vue utilities"), "vue"),
            0.35
        );
    }
}

/// Searches public packages by name or description using bounded offset pagination.
pub async fn search_packages(
    db: &DatabaseConnection,
    query: PackageSearchQuery,
) -> Result<PackageSearchResponse, (StatusCode, String)> {
    let (term, limit, offset) = query.normalized();
    let condition = Condition::any()
        .add(package::Column::FullName.contains(&term))
        .add(package::Column::Name.contains(&term))
        .add(package::Column::Description.contains(&term));

    let selection = Package::find().filter(condition);
    let total = selection
        .clone()
        .count(db)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    let packages = selection
        .order_by_desc(package::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(db)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    let package_ids = packages
        .iter()
        .map(|package| package.id)
        .collect::<Vec<_>>();
    let versions = PackageVersion::find()
        .filter(package_version::Column::PackageId.is_in(package_ids))
        .all(db)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    let mut versions_by_package: HashMap<Uuid, Vec<package_version::Model>> = HashMap::new();
    for version in versions {
        versions_by_package
            .entry(version.package_id)
            .or_default()
            .push(version);
    }

    let objects = packages
        .into_iter()
        .map(|package| {
            let versions = versions_by_package.remove(&package.id).unwrap_or_default();
            let latest = latest_published_version(&versions);
            let version = latest
                .map(|version| version.version.clone())
                .unwrap_or_else(|| "0.0.0".to_string());
            let date = latest
                .map(|version| version.created_at.to_rfc3339())
                .unwrap_or_else(|| package.created_at.to_rfc3339());
            let name = package.full_name.clone();
            let search_score = package_search_score(&name, package.description.as_deref(), &term);

            PackageSearchObject {
                package: PackageSearchPackage {
                    name: name.clone(),
                    version,
                    description: package.description,
                    date,
                    links: PackageSearchLinks {
                        npm: format!("https://fpm.fuyeor.com/package/{name}"),
                    },
                },
                score: PackageSearchScore {
                    final_score: search_score,
                    detail: PackageSearchScoreDetail {
                        quality: 1.0,
                        popularity: 0.0,
                        maintenance: 1.0,
                    },
                },
                search_score,
            }
        })
        .collect();

    Ok(PackageSearchResponse {
        objects,
        total,
        time: chrono::Utc::now().to_rfc3339(),
    })
}

/// Selects the highest valid SemVer and falls back to the newest stored version.
fn latest_published_version(
    versions: &[package_version::Model],
) -> Option<&package_version::Model> {
    let mut latest: Option<(&package_version::Model, Version)> = None;
    for version in versions {
        if let Ok(parsed) = Version::parse(&version.version)
            && latest.as_ref().is_none_or(|(_, current)| parsed > *current)
        {
            latest = Some((version, parsed));
        }
    }
    latest
        .map(|(version, _)| version)
        .or_else(|| versions.iter().max_by_key(|version| version.created_at))
}

/// Produces a deterministic score while keeping npm's quality/popularity/maintenance shape.
fn package_search_score(name: &str, description: Option<&str>, term: &str) -> f64 {
    if term.is_empty() {
        return 0.5;
    }
    let name = name.to_lowercase();
    let term = term.to_lowercase();
    let name_score: f64 = if name == term {
        1.0
    } else if name.starts_with(&term) {
        0.9
    } else if name.contains(&term) {
        0.8
    } else {
        0.0
    };
    let description_score: f64 = description
        .map(str::to_lowercase)
        .filter(|description| description.contains(&term))
        .map_or(0.0, |_| 0.35);
    (name_score + description_score).min(1.0)
}
