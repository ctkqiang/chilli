use axum::Json;
use serde_json::{Value, json};

use crate::{config, models::system_status::SystemStatus};

pub async fn get_system_status() -> Json<SystemStatus> {
    Json(SystemStatus {
        status: "Healthy".to_string(),
        version: config::APP_VERSION.to_string(),
    })
}

pub async fn get_index() -> Json<Value> {
    Json(json!({
        "name": crate::config::APP_NAME,
        "author": crate::config::Author::default(),
        "version": crate::config::APP_VERSION,
    }))
}
