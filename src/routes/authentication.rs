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
