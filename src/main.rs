mod config;
mod models;
mod service;
mod utils;

use crate::models::log_level::LogLevel;
use crate::service::database;

#[tokio::main]
async fn main() {
    utils::logger::log(LogLevel::Info, "正在启动小辣椒服务...");

    let _db = match database::initialise_db().await {
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

    utils::logger::log(LogLevel::Info, "小辣椒服务启动成功！");
}
