// src/modules/user/controller.rs
use super::dto::EmbeddedUserDto;
use crate::entities::{organization_member, prelude::Organization, prelude::User, user};
use crate::modules::{
    auth::middleware::CurrentUser,
    user::dto::{MySessionResponse, UserOrganizationDto, UserPackageDto},
};
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
        (status = 200, body = MySessionResponse),
        (status = 401, description = "Not authenticated")
    ),
    tag = "User",
    security(("token" = []))
)]
/// Get current authenticated user's information
pub async fn get_me(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> Result<Json<MySessionResponse>, (StatusCode, String)> {
    let user_record = User::find_by_id(user.id)
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    let org_records = organization_member::Entity::find()
        .filter(organization_member::Column::UserId.eq(user.id))
        .find_also_related(Organization)
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut mapped_orgs = Vec::new();
    for (member, org_opt) in org_records {
        if let Some(org) = org_opt {
            mapped_orgs.push(UserOrganizationDto {
                id: org.id,
                username: org.username,
                role: member.role,
                created_at: member.created_at,
            });
        }
    }

    Ok(Json(MySessionResponse {
        user: EmbeddedUserDto {
            id: user_record.id,
            username: user_record.username,
            nickname: user_record.nickname,
            avatar: user_record.avatar,
        },
        organizations: mapped_orgs,
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

#[utoipa::path(
    get,
    path = "/users/{username}/organizations",
    responses((status = 200, body = Vec<UserOrganizationDto>)),
    tag = "User"
)]
/// Public API to get all organizations a user belongs to
pub async fn get_user_organizations(
    Path(username): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<UserOrganizationDto>>, (StatusCode, String)> {
    let u = User::find()
        .filter(user::Column::Username.eq(&username))
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let org_records = organization_member::Entity::find()
        .filter(organization_member::Column::UserId.eq(u.id))
        .find_also_related(Organization)
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut mapped_orgs = Vec::new();
    for (member, org_opt) in org_records {
        if let Some(org) = org_opt {
            mapped_orgs.push(UserOrganizationDto {
                id: org.id,
                username: org.username,
                role: member.role,
                created_at: member.created_at,
            });
        }
    }

    Ok(Json(mapped_orgs))
}

#[utoipa::path(
    get,
    path = "/users/{username}/packages",
    responses((status = 200, body = Vec<UserPackageDto>)),
    tag = "User"
)]
/// Public API to get all packages published by this user's scopes
pub async fn get_user_packages(
    Path(username): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<UserPackageDto>>, (StatusCode, String)> {
    let u = User::find()
        .filter(user::Column::Username.eq(&username)) // ✨ 使用最稳妥的精确匹配
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let member_org_ids: Vec<uuid::Uuid> = organization_member::Entity::find()
        .filter(organization_member::Column::UserId.eq(u.id))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_iter()
        .map(|m| m.organization_id)
        .collect();

    let package_records = crate::entities::package::Entity::find()
        .filter(crate::entities::package::Column::OrganizationId.is_in(member_org_ids))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mapped_pkgs = package_records
        .into_iter()
        .map(|pkg| UserPackageDto {
            id: pkg.id,
            name: pkg.name,
            full_name: pkg.full_name,
            description: pkg.description,
            created_at: pkg.created_at,
        })
        .collect();

    Ok(Json(mapped_pkgs))
}
