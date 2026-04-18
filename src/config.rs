use crate::models::log_level::LogLevel;
use crate::utils;
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "(小辣椒&chilli)";
pub const APP_VERSION: &str = "v0.0.1";
pub const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
pub const DEFAULT_SERVER_PORT: u16 = 9333;
pub const GITHUB_ADVISORIES_API_URL: &str = "https://api.github.com/advisories";

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub institution: String,
    pub email: String,
    pub contact: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub github: GithubConfig,
    pub database_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubConfig {
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 9333,
            },
            github: GithubConfig { token: None },
            database_path: get_default_database_path(),
        }
    }
}

impl Default for Author {
    fn default() -> Self {
        Self {
            name: "钟智强".to_string(),
            institution: "哪吒网络安全".to_string(),
            email: "johnmelodymel@qq.com".to_string(),
            contact: "微信: ctkqiang".to_string(),
        }
    }
}

#[allow(dead_code)]
impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config::default())
    }

    #[allow(dead_code)]
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

fn get_default_database_path() -> String {
    let db_path = "./data/chilli.db";

    if let Err(e) = std::fs::create_dir_all("./data") {
        eprintln!("创建数据目录失败: {}", e);
    }

    utils::logger::log(LogLevel::Debug, &format!("数据库路径: {}", db_path));
    format!("sqlite:{}?mode=rwc", db_path)
}

pub fn get_database_path() -> String {
    get_default_database_path()
}
