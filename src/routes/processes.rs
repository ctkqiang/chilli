use axum::Json;

use crate::core::get_running_process::get_running_applicaitons;
use crate::models::system_overview::SystemOverview;

pub async fn runnning_processes() -> Json<SystemOverview> {
    let overview = get_running_applicaitons();
    Json(overview)
}
