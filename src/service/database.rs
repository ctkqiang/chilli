use sea_orm::Statement;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use std::fs;

use crate::config;
use crate::models::log_level::LogLevel;
use crate::utils;

/**
 * 批量创建数据库表宏
 *
 * # 设计意图
 * 这个宏封装了SeaORM的表创建逻辑，提供声明式的方式来批量初始化数据库表结构。
 * 它解决了手动重复编写表创建代码的问题，支持多数据库后端（SQLite/MySQL/PostgreSQL）。
 *
 * # 参数说明
 * - `$db`: 已建立的数据库连接 (DatabaseConnection)
 * - `$backend`: 数据库后端类型 (DatabaseBackend)
 * - `$entities`: 实体类型数组，如 [Entity1, Entity2, ...]
 *
 * # 工作原理
 * ```
 * ┌─────────────────────────────────────────────────────────────┐
 * │                    create_tables! 宏流程                      │
 * ├─────────────────────────────────────────────────────────────┤
 * │  1. 创建 Schema 实例                                         │
 * │     ↓                                                       │
 * │  2. 遍历每个实体类型                                          │
 * │     ↓                                                       │
 * │  3. 生成 CREATE TABLE IF NOT EXISTS 语句                    │
 * │     ↓                                                       │
 * │  4. 执行 SQL 语句                                            │
 * │     ↓                                                       │
 * │  5. 返回 Result<(), DbErr>                                   │
 * └─────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 使用示例
 * ```rust
 * create_tables!(db, backend, [
 *     crate::models::github_advisories::Entity,
 *     crate::models::users::Entity,
 * ]);
 * ```
 *
 * # 技术细节
 * - 使用 `if_not_exists()` 确保幂等性，重复执行不会报错
 * - 宏展开时为每个实体生成独立的执行语句
 * - 任一实体创建失败会立即返回错误
 */
macro_rules! create_tables {
    ($db:expr, $backend:expr, [$( $entity:ty ),* $(,)?]) => {
        let schema = Schema::new($backend);
        $(
            let stmt = $backend.build(
                schema
                    .create_table_from_entity(<$entity>::default())
                    .if_not_exists()
            );
            $db.execute(stmt).await?;
        )*
    };
}

/**
 * 初始化数据库连接并自动创建表结构
 *
 * # 架构设计
 * 本函数实现了多数据库后端支持的连接初始化策略，采用环境变量驱动的配置方式。
 *
 * ```
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                         数据库初始化架构图                               │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────┐ │
 * │   │ POSTGRES_HOST│    │ MYSQL_HOST  │    │QUESTDB_HOST │    │  (默认)  │ │
 * │   └──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └────┬────┘ │
 * │          │                  │                  │                │      │
 * │          ▼                  ▼                  ▼                ▼      │
 * │   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────┐ │
 * │   │ PostgreSQL  │    │   MySQL     │    │  QuestDB    │    │ SQLite  │ │
 * │   │  (优先)     │    │  (次优先)   │    │  (第三)     │    │ (回退)  │ │
 * │   └──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └────┬────┘ │
 * │          │                  │                  │                │      │
 * │          └──────────────────┴──────────────────┴────────────────┘      │
 * │                                     │                                   │
 * │                                     ▼                                   │
 * │                          ┌─────────────────┐                            │
 * │                          │  Database::connect│                           │
 * │                          └────────┬────────┘                            │
 * │                                   │                                     │
 * │                                   ▼                                     │
 * │                          ┌─────────────────┐                            │
 * │                          │ create_tables!  │                            │
 * │                          │    宏执行       │                            │
 * │                          └────────┬────────┘                            │
 * │                                   │                                     │
 * │                                   ▼                                     │
 * │                          ┌─────────────────┐                            │
 * │                          │ 返回 DbConnection│                            │
 * │                          └─────────────────┘                            │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 优先级策略
 * 1. **PostgreSQL**: 检查 `POSTGRES_HOST` 环境变量
 * 2. **MySQL**: 检查 `MYSQL_HOST` 环境变量
 * 3. **QuestDB**: 检查 `QUESTDB_HOST` 环境变量
 * 4. **SQLite**: 默认回退，使用本地文件 `./data/chilli.db`
 *
 * # 环境变量配置
 *
 * ## PostgreSQL
 * - `POSTGRES_HOST`: 数据库主机地址
 * - `POSTGRES_USER`: 用户名 (默认: postgres)
 * - `POSTGRES_PASSWORD`: 密码
 * - `POSTGRES_PORT`: 端口 (默认: 5432)
 * - `POSTGRES_DATABASE`: 数据库名 (默认: chilli)
 *
 * ## MySQL
 * - `MYSQL_HOST`: 数据库主机地址
 * - `MYSQL_USER`: 用户名 (默认: root)
 * - `MYSQL_PASSWORD`: 密码
 * - `MYSQL_PORT`: 端口 (默认: 3306)
 * - `MYSQL_DATABASE`: 数据库名 (默认: chilli)
 *
 * ## QuestDB
 * - `QUESTDB_HOST`: 数据库主机地址
 * - `QUESTDB_USER`: 用户名 (默认: admin)
 * - `QUESTDB_PASSWORD`: 密码 (默认: quest)
 * - `QUESTDB_PORT`: 端口 (默认: 8812)
 *
 * ## SQLite (默认)
 * 无需环境变量，自动创建 `./data/` 目录和 `chilli.db` 文件
 *
 * # 返回值
 * - `Ok(DatabaseConnection)`: 成功建立的数据库连接
 * - `Err(DbErr)`: 连接失败或表创建失败的错误信息
 *
 * # 使用示例
 * ```rust
 * #[tokio::main]
 * async fn main() {
 *     match initialise_db().await {
 *         Ok(db) => {
 *             utils::logger::log(LogLevel::Info, "数据库连接成功!");
 *         }
 *         Err(e) => {
 *             utils::logger::log(LogLevel::Error, &format!("数据库初始化失败: {}", e));
 *             std::process::exit(1);
 *         }
 *     }
 * }
 * ```
 *
 * # 注意事项
 * - 首次使用SQLite时会自动创建数据目录
 * - 所有数据库连接都使用 `create_tables!` 宏自动初始化表结构
 * - 表创建使用 `IF NOT EXISTS`，重复初始化不会报错
 * - 连接字符串使用 `?mode=rwc` 确保SQLite可读写
 */
pub async fn initialise_db() -> Result<DatabaseConnection, DbErr> {
    let (host_opt, mysql_opt, quest_opt) = (
        config::get_env("POSTGRES_HOST"),
        config::get_env("MYSQL_HOST"),
        config::get_env("QUESTDB_HOST"),
    );

    let db_name = if host_opt.is_some() {
        config::get_env("POSTGRES_DATABASE").unwrap_or_else(|| "chilli".to_string())
    } else if mysql_opt.is_some() {
        config::get_env("MYSQL_DATABASE").unwrap_or_else(|| "chilli".to_string())
    } else if quest_opt.is_some() {
        config::get_env("QUESTDB_DATABASE").unwrap_or_else(|| "qdb".to_string())
    } else {
        "chilli".to_string()
    };

    let base_url = match (&host_opt, &mysql_opt, &quest_opt) {
        (Some(host), _, _) => format!(
            "postgres://{}:{}@{}:{}",
            config::get_env("POSTGRES_USER").unwrap_or_else(|| "postgres".to_string()),
            config::get_env("POSTGRES_PASSWORD").unwrap_or_default(),
            host,
            config::get_env("POSTGRES_PORT").unwrap_or_else(|| "5432".to_string())
        ),
        (_, Some(host), _) => format!(
            "mysql://{}:{}@{}:{}",
            config::get_env("MYSQL_USER").unwrap_or_else(|| "root".to_string()),
            config::get_env("MYSQL_PASSWORD").unwrap_or_default(),
            host,
            config::get_env("MYSQL_PORT").unwrap_or_else(|| "3306".to_string())
        ),
        (_, _, Some(host)) => format!(
            "postgres://{}:{}@{}:{}",
            config::get_env("QUESTDB_USER").unwrap_or_else(|| "admin".to_string()),
            config::get_env("QUESTDB_PASSWORD").unwrap_or_else(|| "quest".to_string()),
            host,
            config::get_env("QUESTDB_PORT").unwrap_or_else(|| "8812".to_string())
        ),
        _ => {
            let _ = fs::create_dir_all(config::DATABASE_DIR);
            return Database::connect("sqlite://./data/chilli.db?mode=rwc").await;
        }
    };

    let db = Database::connect(&base_url).await?;
    let db_backend = db.get_database_backend();

    let create_sql = if db_backend == sea_orm::DatabaseBackend::MySql {
        format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name)
    } else {
        format!("CREATE DATABASE \"{}\"", db_name)
    };

    db.execute(Statement::from_string(db_backend, create_sql))
        .await
        .ok();

    let full_url = format!("{}/{}", base_url, db_name);
    let db = Database::connect(&full_url).await?;

    create_tables!(
        db,
        db.get_database_backend(),
        [
            crate::models::github_advisories::Entity,
            crate::models::users::Entity,
        ]
    );

    let backend_name = match db.get_database_backend() {
        sea_orm::DatabaseBackend::MySql => "MYSQL",
        sea_orm::DatabaseBackend::Postgres => "POSTGRESQL",
        sea_orm::DatabaseBackend::Sqlite => "SQLITE",
    };

    utils::logger::log(
        LogLevel::Info,
        &format!("数据库初始化完成 | 驱动程序: {}", backend_name),
    );

    Ok(db)
}
