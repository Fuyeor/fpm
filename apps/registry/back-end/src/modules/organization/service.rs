// src/modules/organization/service.rs
use axum::http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait, TransactionTrait, Set, ActiveModelTrait};
use uuid::Uuid;
use crate::{
    entities::{organization, prelude::Organization, organization_member},
    modules::organization::dto::*,
};

// Hard-coded system-level reserved words
const FORBIDDEN_SCOPES: &[&str] = &[
    "node", "nodejs", "jsr", "deno", "author", "user", "maintainer", 
    "npm", "pnpm", "yarn", "root", "admin", 
    "system", "official", "api", "registry", "auth", "package"
];

/// 验证一个 Scope 名字是否合法且可用
pub async fn check_scope_availability(
    db: &DatabaseConnection,
    name: &str,
    user_id: Option<Uuid>, // 可选：如果登录了，我们可以顺便帮他查额度
) -> Result<ScopeValidationResponse, StatusCode> {
    let lowercased = name.to_lowercase();

    // 1. 格式校验：长度 3 ~ 30，且只能是 a-z, 0-9, -
    let re = regex::Regex::new(r"^[a-z0-9-]{3,30}$").unwrap();
    if !re.is_match(&lowercased) {
        return Ok(ScopeValidationResponse {
            available: false,
            message: "scope.invalid.format".into(),
        });
    }

    // 2. 禁用词校验
    if FORBIDDEN_SCOPES.contains(&lowercased.as_str()) {
        return Ok(ScopeValidationResponse {
            available: false,
            message: "scope.invalid.reserved".into(),
        });
    }

    // 3. 额度校验 (仅当用户登录时触发)
    if let Some(uid) = user_id {
        let owned_count = organization_member::Entity::find()
            .filter(organization_member::Column::UserId.eq(uid))
            .filter(organization_member::Column::Role.eq("admin"))
            .count(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if owned_count >= 5 {
            return Ok(ScopeValidationResponse {
                available: false,
                message: "scope.limit.reached".into(),
            });
        }
    }

    // 4. 重名校验 (查库)
    let exists = Organization::find()
        .filter(organization::Column::Name.eq(&lowercased))
        .one(db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some();

    if exists {
        return Ok(ScopeValidationResponse {
            available: false,
            message: "scope.invalid.taken".into(),
        });
    }

    Ok(ScopeValidationResponse {
        available: true,
        message: "scope.available".into(),
    })
}


pub async fn create_scope(
    db: &DatabaseConnection,
    user_id: Uuid,
    name: String,
) -> Result<CreateScopeResponse, (StatusCode, String)> {
    let lowercased = name.to_lowercase();

    // 再次执行强校验
    let validation = check_scope_availability(db, &lowercased, Some(user_id)).await
        .map_err(|s| (s, "Validation failed".into()))?;
    
    if !validation.available {
        return Err((StatusCode::BAD_REQUEST, validation.message));
    }

    let txn = db.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let org_id = Uuid::now_v7();
    
    // 插入 organization 表
    let org_model = organization::ActiveModel {
        id: Set(org_id),
        name: Set(lowercased.clone()),
        ..Default::default()
    };
    org_model.insert(&txn).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 插入 organization_member 表
    let member_model = organization_member::ActiveModel {
        organization_id: Set(org_id),
        user_id: Set(user_id),
        role: Set("admin".to_string()),
    };
    member_model.insert(&txn).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 提交事务
    txn.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(CreateScopeResponse { id: org_id, name: lowercased })
}