mod config;
mod core;
mod models;
mod routes;
mod service;
mod utils;

use crate::core::get_github_advisories::sync_github_advisories;
use crate::models::log_level::LogLevel;
use crate::service::database;

use axum::routing::get;
use axum::Router;
use std::fs;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = routes();
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
        config::DEFAULT_SERVER_PORT
    ))
    .await
    .unwrap();

    utils::logger::log(
        LogLevel::Info,
        &format!(
            "服务器监听在 http://{}:{}",
            config::DEFAULT_SERVER_HOST,
            config::DEFAULT_SERVER_PORT
        ),
    );

    axum::serve(listener, app).await.unwrap();
}

fn routes() -> Router {
    Router::new()
        .route("/", get(routes::system::get_index))
        .route("/health", get(routes::system::get_system_status))
        .route("/api/running", get(routes::processes::runnning_processes))
        .layer(CorsLayer::permissive())
}
