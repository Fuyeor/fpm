// src/modules/auth/middleware.rs
use super::service::Claims;
use crate::config::AppConfig;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

pub struct CurrentUser {
    pub id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    AppConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Attempt to extract from the Authorization: Bearer header
        // for CLI command line use
        let token_from_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| h[7..].to_string());

        // attempt to extract from HttpOnly cookie (for browser web interface)
        let token = match token_from_header {
            Some(t) => t,
            None => {
                let jar = CookieJar::from_headers(&parts.headers);
                jar.get("access_token")
                    .map(|c| c.value().to_string())
                    .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?
            }
        };

        // verify JWT using the secret key
        let config = AppConfig::from_ref(state);
        let decoding_key = DecodingKey::from_secret(config.jwt_key.as_bytes());

        let token_data =
            decode::<Claims>(&token, &decoding_key, &Validation::default()).map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid or expired token".to_string(),
                )
            })?;

        // return CurrentUser instance
        Ok(CurrentUser {
            id: token_data.claims.sub,
        })
    }
}
