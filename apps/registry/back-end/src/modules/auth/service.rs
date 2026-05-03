// src/modules/auth/service.rs
use super::dto::*;
use crate::{
    config::AppConfig,
    entities::{
        prelude::Token, prelude::User, prelude::UserRefreshToken, token, user, user_refresh_token,
    },
    services::idp,
    utils,
};
use axum::http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

/// Issues dual JWT tokens
pub async fn issue_tokens(
    db: &DatabaseConnection,
    config: &AppConfig,
    user_id: Uuid,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<SigninResponse, StatusCode> {
    let now = chrono::Utc::now();
    let jwt_key = EncodingKey::from_secret(config.jwt_key.as_bytes());

    // 1. Create Access Token (JWT - 20 minutes)
    let access_exp = (now + chrono::Duration::minutes(20)).timestamp() as usize;
    let access_claims = Claims {
        sub: user_id,
        exp: access_exp,
        iat: now.timestamp() as usize,
    };
    let access_token = encode(&Header::default(), &access_claims, &jwt_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Create Refresh Token (JWT - 7 days)
    let refresh_exp = (now + chrono::Duration::days(7)).timestamp() as usize;
    let refresh_claims = Claims {
        sub: user_id,
        exp: refresh_exp,
        iat: now.timestamp() as usize,
    };
    let refresh_token = encode(&Header::default(), &refresh_claims, &jwt_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Save Refresh Token to DB (Main site pixel-perfect logic)
    let refresh_model = user_refresh_token::ActiveModel {
        id: Set(Uuid::now_v7()),
        user_id: Set(user_id),
        token: Set(refresh_token.clone()),
        expires_at: Set(chrono::DateTime::from_timestamp(refresh_exp as i64, 0)
            .unwrap()
            .into()),
        user_agent: Set(user_agent),
        ip_address: Set(ip_address),
        ..Default::default()
    };

    UserRefreshToken::insert(refresh_model)
        .exec(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get latest user info for response
    let u = User::find_by_id(user_id).one(db).await.unwrap().unwrap();

    Ok(SigninResponse {
        access_token,
        refresh_token,
        user: UserDto {
            id: u.id,
            username: u.username,
            nickname: u.nickname,
            avatar: u.avatar,
        },
    })
}

/// Authenticate with IdP and sync user info to local DB
pub async fn signin(
    db: &DatabaseConnection,
    config: &AppConfig,
    code: String,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<SigninResponse, (StatusCode, String)> {
    let http_client = reqwest::Client::new();

    let idp_res = idp::exchange_code(&http_client, config, &code)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let user_uuid = Uuid::parse_str(&idp_res.user.id).unwrap_or_default();

    // Sync user data to local Postgres (UPSERT)
    let user_model = user::ActiveModel {
        id: Set(user_uuid),
        username: Set(idp_res.user.username.clone()),
        nickname: Set(idp_res.user.nickname.clone()),
        avatar: Set(idp_res.user.avatar.clone()),
        updated_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    User::insert(user_model)
        .on_conflict(
            sea_orm::sea_query::OnConflict::column(user::Column::Id)
                .update_columns([
                    user::Column::Username,
                    user::Column::Nickname,
                    user::Column::Avatar,
                    user::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Issue tokens after successful signin
    let res = issue_tokens(db, config, user_uuid, user_agent, ip_address)
        .await
        .map_err(|s| (s, "Token issuance failed".to_string()))?;

    Ok(res)
}

/// JWT Refresh Logic (Rotation)
pub async fn refresh_tokens(
    db: &DatabaseConnection,
    config: &AppConfig,
    old_refresh_token: &str,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<SigninResponse, StatusCode> {
    let decoding_key = DecodingKey::from_secret(config.jwt_key.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = match decode::<Claims>(old_refresh_token, &decoding_key, &validation) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_id = token_data.claims.sub;

    let token_record = UserRefreshToken::find()
        .filter(user_refresh_token::Column::Token.eq(old_refresh_token))
        .filter(user_refresh_token::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Revoke old token before issuing new ones
    token_record
        .delete(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res = issue_tokens(db, config, user_id, user_agent, ip_address).await?;

    Ok(res)
}

/// CLI Personal Access Token creation
pub async fn create_token(
    db: &DatabaseConnection,
    user_id: Uuid,
    name: String,
) -> Result<CreateTokenResponse, (StatusCode, String)> {
    let (plain, hash) = utils::token::generate_fpm_token();

    let token_model = token::ActiveModel {
        id: Set(Uuid::now_v7()),
        user_id: Set(user_id),
        name: Set(name),
        token_hash: Set(hash),
        ..Default::default()
    };

    Token::insert(token_model)
        .exec(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(CreateTokenResponse { token: plain })
}
