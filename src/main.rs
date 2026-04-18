mod config;
mod models;
mod service;
mod utils;

use crate::models::log_level::LogLevel;
use crate::service::database;

#[tokio::main]
async fn main() {
    let db = match database::initialise_db().await {
        Ok(connection) => connection,
        Err(e) => panic!("数据库连接失败: {}", e),
    };

    utils::logger::log(LogLevel::Info, "服务启动成功");
}
