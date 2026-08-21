// src/api/sitemap.rs
//! Database-backed XML sitemap endpoints for public FPM pages.

use axum::{
    extract::State,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use utoipa::OpenApi;

use crate::entities::{
    organization::{Column as OrganizationColumn, Entity as Organization},
    package::{Column as PackageColumn, Entity as Package},
    user::{Column as UserColumn, Entity as User},
};

const PUBLIC_SITE_BASE_URL: &str = "https://fpm.fuyeor.com";
const SITEMAP_XMLNS: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
const URL_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'/');

#[derive(OpenApi)]
#[openapi(paths(get_index, get_users, get_organizations, get_packages))]
pub struct SitemapApi;

#[utoipa::path(
    get,
    path = "/sitemaps/index.xml",
    responses((status = 200, description = "Sitemap index XML")),
    tag = "Sitemap"
)]
/// Returns the English sitemap index and its three entity-specific sitemap URLs.
pub async fn get_index() -> Response {
    xml_response(render_index())
}

#[utoipa::path(
    get,
    path = "/sitemaps/en/users.xml",
    responses((status = 200, description = "User sitemap XML")),
    tag = "Sitemap"
)]
/// Returns public user profile URLs ordered by user creation time.
pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Response, (StatusCode, String)> {
    let users = User::find()
        .order_by_asc(UserColumn::Username)
        .all(&db)
        .await
        .map_err(database_error)?;
    let entries = users.into_iter().map(|user| {
        (
            format!("{PUBLIC_SITE_BASE_URL}/@{}", encode_segment(&user.username)),
            user.created_at.to_rfc3339(),
        )
    });
    Ok(xml_response(render_urlset(entries)))
}

#[utoipa::path(
    get,
    path = "/sitemaps/en/organizations.xml",
    responses((status = 200, description = "Organization sitemap XML")),
    tag = "Sitemap"
)]
/// Returns public organization profile URLs ordered by organization username.
pub async fn get_organizations(
    State(db): State<DatabaseConnection>,
) -> Result<Response, (StatusCode, String)> {
    let organizations = Organization::find()
        .order_by_asc(OrganizationColumn::Username)
        .all(&db)
        .await
        .map_err(database_error)?;
    let entries = organizations.into_iter().map(|organization| {
        (
            format!(
                "{PUBLIC_SITE_BASE_URL}/organization/@{}",
                encode_segment(&organization.username)
            ),
            organization.created_at.to_rfc3339(),
        )
    });
    Ok(xml_response(render_urlset(entries)))
}

#[utoipa::path(
    get,
    path = "/sitemaps/en/packages.xml",
    responses((status = 200, description = "Package sitemap XML")),
    tag = "Sitemap"
)]
/// Returns public package profile URLs ordered by package full name.
pub async fn get_packages(
    State(db): State<DatabaseConnection>,
) -> Result<Response, (StatusCode, String)> {
    let packages = Package::find()
        .order_by_asc(PackageColumn::FullName)
        .all(&db)
        .await
        .map_err(database_error)?;
    let entries = packages.into_iter().map(|package| {
        let (scope, name) = package.full_name.split_once('/').ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid package full name: {}", package.full_name),
            )
        })?;
        Ok::<_, (StatusCode, String)>((
            format!(
                "{PUBLIC_SITE_BASE_URL}/package/{}/{}",
                encode_segment(scope),
                encode_segment(name)
            ),
            package.created_at.to_rfc3339(),
        ))
    });
    let entries = entries.collect::<Result<Vec<_>, _>>()?;
    Ok(xml_response(render_urlset(entries)))
}

fn render_index() -> String {
    let locations = [
        format!("{PUBLIC_SITE_BASE_URL}/sitemaps/en/users.xml"),
        format!("{PUBLIC_SITE_BASE_URL}/sitemaps/en/organizations.xml"),
        format!("{PUBLIC_SITE_BASE_URL}/sitemaps/en/packages.xml"),
    ];
    let mut xml = xml_header("sitemapindex");
    for location in locations {
        xml.push_str("<sitemap><loc>");
        xml.push_str(&escape_xml(&location));
        xml.push_str("</loc></sitemap>");
    }
    xml.push_str("</sitemapindex>");
    xml
}

fn render_urlset<I>(entries: I) -> String
where
    I: IntoIterator<Item = (String, String)>,
{
    let mut xml = xml_header("urlset");
    for (location, lastmod) in entries {
        xml.push_str("<url><loc>");
        xml.push_str(&escape_xml(&location));
        xml.push_str("</loc><lastmod>");
        xml.push_str(&escape_xml(&lastmod));
        xml.push_str("</lastmod></url>");
    }
    xml.push_str("</urlset>");
    xml
}

fn xml_header(root: &str) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><{root} xmlns=\"{SITEMAP_XMLNS}\">")
}

fn encode_segment(value: &str) -> String {
    utf8_percent_encode(value, URL_SEGMENT_ENCODE_SET).to_string()
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn xml_response(body: String) -> Response {
    let mut response = body.into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/xml; charset=utf-8"),
    );
    response
}

fn database_error(error: sea_orm::DbErr) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to generate sitemap: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::{escape_xml, render_index, render_urlset};

    #[test]
    fn escapes_xml_values() {
        assert_eq!(escape_xml("a&b<c>\"d'"), "a&amp;b&lt;c&gt;&quot;d&apos;");
    }

    #[test]
    fn renders_index_with_requested_english_sitemaps() {
        let xml = render_index();
        assert!(xml.contains("/sitemaps/en/users.xml"));
        assert!(xml.contains("/sitemaps/en/organizations.xml"));
        assert!(xml.contains("/sitemaps/en/packages.xml"));
    }

    #[test]
    fn renders_urlset_with_lastmod() {
        let xml = render_urlset([(
            "https://fpm.fuyeor.com/@alice".to_string(),
            "2026-08-22T00:00:00Z".to_string(),
        )]);
        assert!(xml.starts_with("<?xml version=\"1.0\""));
        assert!(xml.contains("<url><loc>https://fpm.fuyeor.com/@alice</loc><lastmod>2026-08-22T00:00:00Z</lastmod></url>"));
    }
}
