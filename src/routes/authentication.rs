use crate::core::get_authentication;
use crate::models::users;
use axum::{
    extract::Request, http::StatusCode, middleware::Next, response::Response, Extension, Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

/**
 * 注册
 *
 * 本路由用于用户注册。
 * 它从请求体中提取用户名和密码，将密码哈希化后存储到数据库中。
 */
pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> StatusCode {
    let password_hash = crate::models::hash_password(&payload.password);

    let new_user = users::ActiveModel {
        username: Set(payload.username),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    match new_user.insert(&db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

/**
 * 登录
 *
 * 本路由用于用户登录。
 * 它从请求体中提取用户名和密码，验证密码是否正确，然后根据用户名创建一个 JWT Token。
 */
pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if crate::models::verify_password(&payload.password, &user.password_hash) {
        let token = get_authentication::create_token(&user.username);
        Ok(Json(TokenResponse { token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/**
 * 删除用户
 *
 * 本路由用于删除当前认证的用户。
 * 它从请求头中提取 Bearer Token，验证 Token 是否有效，然后根据 Token 中的用户名 ID 删除对应的用户记录。
 */
pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    headers: axum::http::HeaderMap,
) -> StatusCode {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        if let Ok(username) = get_authentication::verify_token(token) {
            let _ = users::Entity::delete_many()
                .filter(users::Column::Username.eq(username))
                .exec(&db)
                .await;
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::UNAUTHORIZED
}

/**
 * 认证中间件
 *
 * 本中间件用于验证请求头中的 Bearer Token 是否有效。
 * 如果 Token 有效，将用户 ID 从 Token 中提取出来，并将用户 ID 作为请求上下文传递给下一个处理函数。
 * 如果 Token 无效，返回 401 Unauthorized 响应。
 */
#[allow(unused)]
pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .filter(|s: &&str| s.starts_with("Bearer "));

    if let Some(header) = auth_header {
        let token = &header[7..];

        if get_authentication::verify_token(token).is_ok() {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
