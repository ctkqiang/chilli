mod config;
mod core;
mod models;
mod routes;
mod service;
mod utils;

use crate::core::get_github_advisories::sync_github_advisories;
use crate::models::log_level::LogLevel;
use crate::service::database;
use std::process::{Child, Command, Stdio};

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;

struct ProcessGuard(Child);

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

#[tokio::main]
async fn main() {
    let app = routes();
    let _portal_guard = ProcessGuard(launch_portal());

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

fn routes() -> Router {
    Router::new()
        .route("/", get(routes::system::get_index))
        .route("/health", get(routes::system::get_system_status))
        .route("/api/running", get(routes::processes::runnning_processes))
        .route("/api/kill/:pid", post(routes::processes::kill_process))
        .layer(CorsLayer::permissive())
}

use which::which;

pub fn launch_portal() -> Child {
    let portal_dir = "../portal";

    let (runner, args) = if which("bun").is_ok() {
        (
            "bun",
            format!("run dev --port {}", config::Port::default().portal),
        )
    } else if which("npm").is_ok() {
        (
            "npm",
            format!("run dev -- --port {}", config::Port::default().portal),
        )
    } else {
        panic!(
            "\n\n[Chilli 环境缺失]: 找不到 Bun 或 Node.js (npm)! \n\
            请安装其中之一以启动前端门户。\n\
            Bun: https://bun.sh\n\
            Node.js: https://nodejs.org\n"
        );
    };

    #[cfg(not(windows))]
    let (shell, flag) = ("sh", "-c");

    #[cfg(windows)]
    let (shell, flag) = ("cmd", "/C");

    let full_command = format!("{} {}", runner, args);

    utils::logger::log(
        crate::models::log_level::LogLevel::Info,
        &format!(
            "正在使用 {} 启动前端... {}",
            runner,
            format!(
                "http://{}:{}",
                config::DEFAULT_SERVER_HOST,
                config::Port::default().portal
            )
        ),
    );

    Command::new(shell)
        .args([flag, &full_command])
        .current_dir(portal_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect(&format!(
            "[Chilli 错误]: 无法在前端目录 '{}' 中启动门户，目录是否存在？",
            portal_dir
        ))
}
