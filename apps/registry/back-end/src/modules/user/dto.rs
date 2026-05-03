// src/modules/user/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedUserDto {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}
