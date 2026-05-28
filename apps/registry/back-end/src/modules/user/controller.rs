// src/modules/user/controller.rs
use super::dto::EmbeddedUserDto;
use crate::entities::{prelude::User, user};
use crate::modules::auth::middleware::CurrentUser;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, body = EmbeddedUserDto),
        (status = 401, description = "Not authenticated")
    ),
    tag = "User",
    security(("token" = []))
)]
/// Get current authenticated user's information
pub async fn get_me(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> Result<Json<EmbeddedUserDto>, (StatusCode, String)> {
    let user = User::find_by_id(user.id)
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    Ok(Json(EmbeddedUserDto {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        avatar: user.avatar,
    }))
}

#[utoipa::path(
    get,
    path = "/users/{username}",
    responses(
        (status = 200, body = EmbeddedUserDto),
        (status = 404, description = "User not found")
    ),
    tag = "User"
)]
/// Public API to get a user profile by username
pub async fn get_user_profile(
    Path(username): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<EmbeddedUserDto>, (StatusCode, String)> {
    let user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(EmbeddedUserDto {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        avatar: user.avatar,
    }))
}
