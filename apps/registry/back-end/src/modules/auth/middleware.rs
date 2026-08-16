// src/modules/auth/middleware.rs
use super::service::Claims;
use crate::{
    config::AppConfig,
    entities::{prelude::Token, token},
    utils::token::hash_token,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct CurrentUser {
    pub id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    AppConfig: FromRef<S>,
    DatabaseConnection: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token_from_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(str::to_owned);

        let token = match token_from_header {
            Some(token) => token,
            None => {
                let jar = CookieJar::from_headers(&parts.headers);
                jar.get("access_token")
                    .map(|cookie| cookie.value().to_string())
                    .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?
            }
        };

        // Personal tokens are opaque credentials and must be checked against the revocation table.
        if token.starts_with("fpm_") {
            let token_hash = hash_token(&token);
            let db = DatabaseConnection::from_ref(state);
            let record = Token::find()
                .filter(token::Column::TokenHash.eq(token_hash))
                .one(&db)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to validate personal token".to_string(),
                    )
                })?
                .ok_or((
                    StatusCode::UNAUTHORIZED,
                    "Invalid or revoked personal token".to_string(),
                ))?;

            return Ok(CurrentUser { id: record.user_id });
        }

        // Web sessions continue to use the existing signed access JWT.
        let config = AppConfig::from_ref(state);
        let decoding_key = DecodingKey::from_secret(config.jwt_key.as_bytes());
        let token_data =
            decode::<Claims>(&token, &decoding_key, &Validation::default()).map_err(|_| {
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
