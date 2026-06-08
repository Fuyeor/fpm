// src/modules/auth/controller.rs
use super::{dto::*, middleware::CurrentUser, service};
use crate::config::AppConfig;
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode, header},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/auth/signin",
    request_body = SigninRequest,
    responses((status = 200, body = SigninResponse)),
    tag = "Auth"
)]
/// Signin via IdP and sync user info
pub async fn signin(
    State(db): State<DatabaseConnection>,
    State(config): State<AppConfig>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Json(payload): Json<SigninRequest>,
) -> Result<(CookieJar, Json<SigninResponse>), (StatusCode, String)> {
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let ip_address = Some(addr.ip().to_string());

    let res = service::signin(&db, &config, payload.code, user_agent, ip_address).await?;

    let jar = CookieJar::new()
        .add(make_cookie(
            "access_token",
            res.access_token.clone(),
            time::Duration::minutes(20),
            true, // HttpOnly = true (Secure)
        ))
        .add(make_cookie(
            "refresh_token",
            res.refresh_token.clone(),
            time::Duration::days(7),
            true, // HttpOnly = true (Secure)
        ))
        .add(make_cookie(
            "session_payload",
            "true".to_string(),
            time::Duration::days(7),
            false, // HttpOnly = false
        ));

    Ok((jar, Json(res)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh-token",
    responses((status = 200, body = SigninResponse)),
    tag = "Auth"
)]
/// refresh access token using refresh token
pub async fn refresh(
    State(db): State<DatabaseConnection>,
    State(config): State<AppConfig>,
    jar: CookieJar,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
) -> Result<(CookieJar, Json<SigninResponse>), (StatusCode, String)> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing refresh token".to_string(),
        ))?;

    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let ip_address = Some(addr.ip().to_string());

    let res = service::refresh_tokens(&db, &config, &refresh_token, user_agent, ip_address)
        .await
        .map_err(|s| (s, "Refresh failed".to_string()))?;

    let updated_jar = jar
        .add(make_cookie(
            "access_token",
            res.access_token.clone(),
            time::Duration::minutes(20),
            true,
        ))
        .add(make_cookie(
            "refresh_token",
            res.refresh_token.clone(),
            time::Duration::days(7),
            true,
        ))
        .add(make_cookie(
            "session_payload",
            "true".to_string(),
            time::Duration::days(7),
            false, // HttpOnly = false
        ));

    Ok((updated_jar, Json(res)))
}

#[utoipa::path(
    post,
    path = "/auth/token",
    request_body = CreateTokenRequest,
    responses((status = 200, body = CreateTokenResponse)),
    tag = "Auth",
    security(("token" = []))
)]
/// Create a personal access token for CLI
pub async fn create_token(
    State(db): State<DatabaseConnection>,
    user: CurrentUser,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, (StatusCode, String)> {
    service::create_token(&db, user.id, payload.name)
        .await
        .map(Json)
}

#[utoipa::path(
    get,
    path = "/auth/tokens",
    responses((status = 200, body = Vec<UserTokenDto>)),
    tag = "Auth",
    security(("token" = []))
)]
/// List all personal access tokens
pub async fn list_tokens(
    State(db): State<DatabaseConnection>,
    user: CurrentUser,
) -> Result<Json<Vec<UserTokenDto>>, (StatusCode, String)> {
    service::list_tokens(&db, user.id)
        .await
        .map(Json)
        .map_err(|s| (s, "Failed to list tokens".into()))
}

#[utoipa::path(
    delete,
    path = "/auth/tokens/{id}",
    responses((status = 204, description = "Token revoked")),
    tag = "Auth",
    security(("token" = []))
)]
/// Revoke a personal access token
pub async fn revoke_token(
    State(db): State<DatabaseConnection>,
    user: CurrentUser,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    service::revoke_token(&db, user.id, id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|s| (s, "Failed to revoke token".into()))
}

fn make_cookie(
    name: &'static str,
    value: String,
    age: time::Duration,
    http_only: bool,
) -> Cookie<'static> {
    Cookie::build((name, value))
        .http_only(http_only)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(age)
        .build()
}
