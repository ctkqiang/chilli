mod config;
mod models;
mod utils;

use crate::models::log_level::LogLevel;

fn main() {
    utils::logger::log(LogLevel::Info, "服务启动成功");
}
