// src/api/sitemap.rs
//! Database-backed, locale-aware XML sitemap endpoints for public FPM pages.

use axum::{
    extract::{Path, State},
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
const PUBLIC_LOCALES: &[&str] = &[
    "en", "fr", "es", "pt", "de", "ar", "ru", "ja", "ko", "zh-hans", "zh-hant",
];
const SITEMAP_MODULES: &[&str] = &["users", "organizations", "packages"];
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
#[openapi(paths(get_index, get_localized_sitemap))]
pub struct SitemapApi;

#[utoipa::path(
    get,
    path = "/sitemaps/index.xml",
    responses((status = 200, description = "Locale-aware sitemap index XML")),
    tag = "Sitemap"
)]
/// Returns the sitemap index for every public locale and sitemap module.
pub async fn get_index() -> Response {
    xml_response(render_index())
}

#[utoipa::path(
    get,
    path = "/sitemaps/{locale}/{module}",
    params(
        ("locale" = String, Path, description = "Public locale, for example en or zh-hant"),
        ("module" = String, Path, description = "Sitemap XML file: users.xml, organizations.xml, or packages.xml")
    ),
    responses((status = 200, description = "Localized sitemap XML")),
    tag = "Sitemap"
)]
/// Returns one locale-aware sitemap XML file for users, organizations, or packages.
pub async fn get_localized_sitemap(
    State(db): State<DatabaseConnection>,
    Path((raw_locale, module)): Path<(String, String)>,
) -> Result<Response, (StatusCode, String)> {
    let locale = normalize_locale(&raw_locale).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Unsupported sitemap locale: {raw_locale}"),
        )
    })?;

    match module.as_str() {
        "users.xml" => generate_users_sitemap(&db, locale).await,
        "organizations.xml" => generate_organizations_sitemap(&db, locale).await,
        "packages.xml" => generate_packages_sitemap(&db, locale).await,
        _ => Err((
            StatusCode::NOT_FOUND,
            format!("Invalid sitemap module: {module}"),
        )),
    }
}

/// Generates the localized public user profile sitemap.
async fn generate_users_sitemap(
    db: &DatabaseConnection,
    locale: &str,
) -> Result<Response, (StatusCode, String)> {
    let users = User::find()
        .order_by_asc(UserColumn::Username)
        .all(db)
        .await
        .map_err(database_error)?;
    let entries = users.into_iter().map(|user| {
        (
            localized_url(locale, &format!("/@{}", encode_segment(&user.username))),
            user.created_at.to_rfc3339(),
        )
    });
    Ok(xml_response(render_urlset(entries)))
}

/// Generates the localized public organization profile sitemap.
async fn generate_organizations_sitemap(
    db: &DatabaseConnection,
    locale: &str,
) -> Result<Response, (StatusCode, String)> {
    let organizations = Organization::find()
        .order_by_asc(OrganizationColumn::Username)
        .all(db)
        .await
        .map_err(database_error)?;
    let entries = organizations.into_iter().map(|organization| {
        (
            localized_url(
                locale,
                &format!("/organization/@{}", encode_segment(&organization.username)),
            ),
            organization.created_at.to_rfc3339(),
        )
    });
    Ok(xml_response(render_urlset(entries)))
}

/// Generates the localized public package profile sitemap.
async fn generate_packages_sitemap(
    db: &DatabaseConnection,
    locale: &str,
) -> Result<Response, (StatusCode, String)> {
    let packages = Package::find()
        .order_by_asc(PackageColumn::FullName)
        .all(db)
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
            localized_url(
                locale,
                &format!(
                    "/package/{}/{}",
                    encode_segment(scope),
                    encode_segment(name)
                ),
            ),
            package.created_at.to_rfc3339(),
        ))
    });
    let entries = entries.collect::<Result<Vec<_>, _>>()?;
    Ok(xml_response(render_urlset(entries)))
}

fn render_index() -> String {
    let mut xml = xml_header("sitemapindex");
    for locale in PUBLIC_LOCALES {
        for module in SITEMAP_MODULES {
            xml.push_str("<sitemap><loc>");
            xml.push_str(&escape_xml(&format!(
                "{PUBLIC_SITE_BASE_URL}/sitemaps/{locale}/{module}.xml"
            )));
            xml.push_str("</loc></sitemap>");
        }
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

fn normalize_locale(raw_locale: &str) -> Option<&'static str> {
    let normalized = raw_locale.to_ascii_lowercase();
    PUBLIC_LOCALES
        .iter()
        .copied()
        .find(|locale| *locale == normalized)
}

fn localized_url(locale: &str, path: &str) -> String {
    format!("{PUBLIC_SITE_BASE_URL}/{locale}{path}")
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
    use super::{
        PUBLIC_LOCALES, SITEMAP_MODULES, escape_xml, normalize_locale, render_index, render_urlset,
    };

    #[test]
    fn escapes_xml_values() {
        assert_eq!(escape_xml("a&b<c>\"d'"), "a&amp;b&lt;c&gt;&quot;d&apos;");
    }

    #[test]
    fn normalizes_supported_locales_and_rejects_unknown_values() {
        assert_eq!(normalize_locale("ZH-Hant"), Some("zh-hant"));
        assert_eq!(normalize_locale("en"), Some("en"));
        assert_eq!(normalize_locale("xx"), None);
    }

    #[test]
    fn renders_index_for_every_locale_and_module() {
        let xml = render_index();
        for locale in PUBLIC_LOCALES {
            for module in SITEMAP_MODULES {
                assert!(xml.contains(&format!("/sitemaps/{locale}/{module}.xml")));
            }
        }
    }

    #[test]
    fn renders_localized_urlset_with_lastmod() {
        let xml = render_urlset([(
            "https://fpm.fuyeor.com/zh-hant/@alice".to_string(),
            "2026-08-22T00:00:00Z".to_string(),
        )]);
        assert!(xml.starts_with("<?xml version=\"1.0\""));
        assert!(xml.contains(
            "<url><loc>https://fpm.fuyeor.com/zh-hant/@alice</loc><lastmod>2026-08-22T00:00:00Z</lastmod></url>"
        ));
    }
}
