mod config;
mod core;
mod models;
mod routes;
mod service;
mod utils;

use sea_orm::DatabaseConnection;
use crate::core::get_github_advisories::sync_github_advisories;
use crate::models::log_level::LogLevel;
use crate::service::database;

use axum::extract::Extension;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    utils::logger::log(LogLevel::Info, "正在启动小辣椒服务...");

    let db = match database::initialise_db().await {
        Ok(connection) => {
            utils::logger::log(LogLevel::Info, "数据库连接成功");
            connection
        }

        Err(e) => {
            utils::logger::log(LogLevel::Error, &format!("数据库连接失败: {}", e));
            utils::logger::log(LogLevel::Info, "提示: 请确保 data 目录有写入权限");
            panic!("服务启动失败，请检查数据库配置");
        }
    };
    let app = routes(db.clone());

    match sync_github_advisories(&db).await {
        Ok(_) => {
            utils::logger::log(LogLevel::Info, "安全公告同步完成");
        }
        Err(e) => {
            utils::logger::log(
                LogLevel::Warn,
                &format!("GitHub 同步失败 (可能达到速率限制): {}", e),
            );
        }
    }

    utils::logger::log(LogLevel::Info, "小辣椒服务启动成功！");

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config::DEFAULT_SERVER_HOST,
        config::Port::default().core,
    ))
    .await
    .unwrap();

    utils::logger::log(
        LogLevel::Info,
        &format!(
            "服务器监听在 http://{}:{}",
            config::DEFAULT_SERVER_HOST,
            config::Port::default().core,
        ),
    );

    axum::serve(listener, app).await.unwrap();
}

fn routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(routes::system::get_index))
        .route("/health", get(routes::system::get_system_status))
        .route("/api/auth/register", post(routes::authentication::register))
        .route("/api/auth/login", post(routes::authentication::login))
        .route(
            "/api/auth/remove",
            post(routes::authentication::delete_user),
        )
        .route("/api/running", get(routes::processes::runnning_processes))
        .route("/api/kill/:pid", post(routes::processes::kill_process))
        .route(
            "/api/security/scan",
            get(routes::security::scan_vulnerabilities),
        )
        .route(
            "/api/security/docker",
            get(routes::security::scan_docker_security),
        )
        .layer(Extension(db))
        .layer(CorsLayer::permissive())
}
