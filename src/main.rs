mod config;
mod core;
mod ip_monitor;
mod models;
mod routes;
mod service;
mod utils;

use crate::core::get_github_advisories::sync_github_advisories;
use crate::models::log_level::LogLevel;
use crate::service::database;
use sea_orm::DatabaseConnection;

use axum::extract::Extension;
use axum::routing::{get, post};
use axum::Router;
use std::process::Command as StdCommand;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    utils::logger::log(LogLevel::Info, "正在启动小辣椒服务...");

    let portal_dir = config::get_portal_dir();
    match StdCommand::new("bun")
        .arg("run")
        .arg("dev")
        .current_dir(&portal_dir)
        .spawn()
    {
        Ok(_child) => {
            utils::logger::log(
                LogLevel::Info,
                &format!("Portal (Vite) 开发服务器 → http://localhost:3000"),
            );
        }
        Err(e) => {
            utils::logger::log(
                LogLevel::Warn,
                &format!("Portal 启动失败 (bun 未安装或路径错误): {}", e),
            );
        }
    }

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

    ip_monitor::start_ip_monitor(db.clone());
    let app = routes(db.clone(), &portal_dir);

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
            "API 服务器 → http://{}:{}",
            config::DEFAULT_SERVER_HOST,
            config::Port::default().core,
        ),
    );

    axum::serve(listener, app).await.unwrap();
}

fn routes(db: DatabaseConnection, portal_dir: &str) -> Router {
    let dist_dir = format!("{}/dist", portal_dir);

    Router::new()
        .route("/health", get(routes::system::get_system_status))
        .route("/api/info", get(routes::system::get_index))
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
        .route(
            "/api/ip-access-logs",
            get(routes::ip_access::get_ip_access_logs),
        )
        .layer(Extension(db))
        .layer(CorsLayer::permissive())
        .fallback_service(
            ServeDir::new(&dist_dir)
                .not_found_service(ServeFile::new(format!("{}/index.html", dist_dir))),
        )
}
