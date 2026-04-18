use crate::models::log_level::LogLevel;
use crate::utils;

pub const APP_NAME: &str = "(小辣椒&chilli)";
pub const APP_VERSION: &str = "v0.0.1";

pub const GITHUB_ADVISORIES_API_URL: &str = "https://api.github.com/advisories";

pub fn get_database_path() -> String {
    let db_path = "./data/chilli.db";

    if let Err(e) = std::fs::create_dir_all("./data") {
        eprintln!("创建数据目录失败: {}", e);
    }

    utils::logger::log(LogLevel::Debug, &format!("数据库路径: {}", db_path));
    format!("sqlite:{}?mode=rwc", db_path)
}
