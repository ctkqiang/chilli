use axum::extract::{Extension, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IpAccessLogsQuery {
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_limit() -> u32 {
    50
}

pub async fn get_ip_access_logs(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<IpAccessLogsQuery>,
) -> impl IntoResponse {
    let limit = if params.limit == 0 || params.limit > 1000 {
        50
    } else {
        params.limit
    };

    match crate::service::database::get_recent_access_logs(&db, limit).await {
        Ok(logs) => (StatusCode::OK, Json(logs)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}
