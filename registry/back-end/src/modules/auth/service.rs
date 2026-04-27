// src/modules/auth/service.rs
use super::dto::*;
use crate::config::AppConfig;
use crate::entities::{prelude::User, token, user};
use crate::services::idp;
use crate::utils;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

/// Authenticate with IdP and sync user info to local DB
pub async fn signin(
    db: &DatabaseConnection,
    config: &AppConfig,
    code: String,
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

    Ok(SigninResponse {
        access_token: idp_res.access_token,
        user: UserResponse {
            id: user_uuid,
            username: idp_res.user.username,
            nickname: idp_res.user.nickname,
            avatar: idp_res.user.avatar,
        },
    })
}

/// Generate and store a new hashed token for CLI
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

    token_model
        .insert(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(CreateTokenResponse { token: plain })
}
