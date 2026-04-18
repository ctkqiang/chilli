use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::core::get_running_process::get_running_applicaitons;
use crate::models::system_overview::SystemOverview;

pub async fn runnning_processes() -> Json<SystemOverview> {
    let overview = get_running_applicaitons();
    Json(overview)
}

pub async fn kill_process(Path(pid): Path<u32>) -> impl IntoResponse {
    match crate::core::get_running_process::kill_process_by_pid(pid) {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "status": "success", "pid": pid })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
        ),
    }
}
