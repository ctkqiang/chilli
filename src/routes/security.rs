use crate::core::get_security::check_vulnerability_for_app;
use crate::models::github_advisories::ScanRequest;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DatabaseConnection;
use serde_json::json;

#[allow(unused)]
pub async fn scan_vulnerabilities(
    Extension(_db): Extension<DatabaseConnection>,
    Json(req): Json<ScanRequest>,
) -> impl IntoResponse {
    let issues = match check_vulnerability_for_app(&req.package, &req.version, &req.ecosystem).await
    {
        Ok(issues) => issues,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("扫描失败: {}", e) })),
            )
                .into_response();
        }
    };

    (StatusCode::OK, Json(issues)).into_response()
}
