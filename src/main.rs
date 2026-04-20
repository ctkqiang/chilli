mod config;
mod core;
mod models;
mod routes;
mod service;
mod utils;

use sea_orm::DatabaseConnection;
use which::which;

use crate::core::get_github_advisories::sync_github_advisories;
use crate::models::log_level::LogLevel;
use crate::service::database;
use std::process::{Child, Command, Stdio};

use axum::extract::Extension;
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

/**
 * 路由配置中心
 * * 这里的架构将 API 分为 [公开接口] 和 [受保护接口]：
 * 1. 公开接口：用于系统监控、健康检查以及身份验证（注册/登录）。
 * 2. 受保护接口：通过 `auth_middleware` 拦截，必须持有有效的 PASETO Token 才能访问。
 *
 * # 权限校验流程
 * 客户端请求 -> 检查 Authorization Header -> Bearer <Token> -> 解密 PASETO -> 验证通过 -> 执行业务
 *
 * # API 调用示例
 *
 * ## 获取系统进程（需要认证）
 * ```bash
 * curl -X GET http://localhost:9333/api/running \
 *   -H "Authorization: Bearer <YOUR_TOKEN>"
 * ```
 *
 * ## 终止指定进程（需要认证）
 * ```bash
 * curl -X POST http://localhost:9333/api/kill/1234 \
 *   -H "Authorization: Bearer <YOUR_TOKEN>"
 * ```
 *
 * ## 用户注册
 * ```bash
 * curl -X POST http://localhost:9333/api/auth/register \
 *   -H "Content-Type: application/json" \
 *   -d '{"username":"admin","password":"secure123"}'
 * ```
 *
 * ## 用户登录
 * ```bash
 * curl -X POST http://localhost:9333/api/auth/login \
 *   -H "Content-Type: application/json" \
 *   -d '{"username":"admin","password":"secure123"}'
 * ```
 *
 * ## 删除用户（需要认证）
 * ```bash
 * curl -X POST http://localhost:9333/api/auth/remove \
 *   -H "Authorization: Bearer <YOUR_TOKEN>"
 * ```
 *
 * ## 健康检查
 * ```bash
 * curl -X GET http://localhost:9333/health
 * ```
 *
 * ## 系统索引
 * ```bash
 * curl -X GET http://localhost:9333/
 * ```
 */
fn routes(db: DatabaseConnection) -> Router {
    // let protected_routes = Router::new()
    //     .route("/api/running", get(routes::processes::runnning_processes))
    //     .route("/api/kill/:pid", post(routes::processes::kill_process))
    //     .layer(axum::middleware::from_fn(auth_middleware));

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

/**
 * 启动前端门户 (Portal)
 *
 * # 功能概述
 * 自动检测并启动前端开发服务器。支持 Bun 和 Node.js (npm) 两种运行时环境，
 * 优先使用 Bun（性能更优），回退到 npm。
 *
 * # 启动流程
 * ```
 * 检测 Bun -> 存在？使用 bun run dev
 *    |
 *    v
 * 检测 npm -> 存在？使用 npm run dev
 *    |
 *    v
 *  都不存在 -> panic 提示安装
 * ```
 *
 * # 跨平台支持
 * - Linux/macOS: 使用 `sh -c` 执行命令
 * - Windows: 使用 `cmd /C` 执行命令
 *
 * # 目录结构要求
 * ```
 * chilli/
 * ├── src/           # 后端代码
 * └── ../portal/     # 前端代码 (相对路径)
 *     ├── package.json
 *     └── ...
 * ```
 *
 * # 进程管理
 * 返回的 `Child` 进程由 `ProcessGuard` 包装，当后端服务退出时，
 * 前端进程会自动被终止（通过 Drop trait 实现）。
 *
 * # 错误处理
 * - 找不到 Bun/npm: panic 并提示安装链接
 * - 启动失败: panic 并检查 portal 目录是否存在
 *
 * # 日志输出ZZz
 * 启动成功后会记录日志：
 * `[INFO] 正在使用 {runner} 启动前端... http://127.0.0.1:{portal_port}`
 *
 * # 依赖
 * - `which` crate: 检测系统命令是否存在
 * - `config::Port`: 获取配置的端口号
 *
 * # 示例
 * ```rust
 * let portal_guard = ProcessGuard(launch_portal());
 * -> 前端现在运行在 http://127.0.0.1:3000
 * ```
 */
pub fn launch_portal() -> Child {
    let portal_dir = config::PORTAL_DIR;

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
