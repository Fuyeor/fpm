// src/modules/auth/middleware.rs
use super::service::Claims;
use crate::config::AppConfig;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
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
        // Get Bearer token
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| &h[7..])
            .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

        let config = AppConfig::from_ref(state);
        let decoding_key = DecodingKey::from_secret(config.jwt_key.as_bytes());

        // extract user ID from claims and return CurrentUser
        let token_data = decode::<Claims>(auth_header, &decoding_key, &Validation::default())
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid or expired token".to_string(),
                )
            })?;

        Ok(CurrentUser {
            id: token_data.claims.sub,
        })
    }
}
