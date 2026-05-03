// src/modules/user/controller.rs
use super::dto::EmbeddedUserDto;
use crate::entities::prelude::User;
use crate::modules::auth::middleware::CurrentUser;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

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
