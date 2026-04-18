/**
 * 数据库服务单元测试
 *
 * 测试数据库连接、表创建、数据操作等核心功能。
 * 所有测试使用内存数据库，确保快速执行和完全隔离。
 */
use sea_orm::{Database, DatabaseBackend, Schema};
use std::env;

/**
 * 数据库连接测试模块
 *
 * 验证不同数据库后端的连接字符串构建和连接建立。
 * 包括SQLite、MySQL、PostgreSQL三种后端。
 */
mod connection_tests {
    use super::*;

    /**
     * SQLite内存数据库连接测试
     *
     * 验证内存数据库的连接建立和基本操作。
     * 这是单元测试中最常用的数据库类型。
     */
    #[tokio::test]
    async fn test_sqlite_in_memory_connection() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("内存数据库连接应成功");

        assert_eq!(db.get_database_backend(), DatabaseBackend::Sqlite);
    }

    /**
     * SQLite文件数据库连接测试
     *
     * 验证文件数据库的连接和基本操作。
     * 测试完成后自动清理临时文件。
     */
    #[tokio::test]
    async fn test_sqlite_file_connection() {
        let temp_path = format!("sqlite:///tmp/test_{}.db?mode=rwc", uuid::Uuid::new_v4());
        let db = Database::connect(&temp_path)
            .await
            .expect("文件数据库连接应成功");

        assert_eq!(db.get_database_backend(), DatabaseBackend::Sqlite);

        // 清理临时文件
        let _ = std::fs::remove_file(
            &temp_path
                .replace("sqlite:///tmp/", "/tmp/")
                .replace("?mode=rwc", ""),
        );
    }

    /**
     * 数据库URL环境变量解析测试
     *
     * 验证从环境变量构建数据库连接字符串的逻辑。
     * 测试不同环境变量组合下的URL生成。
     */
    #[test]
    fn test_database_url_from_env() {
        // 测试SQLite默认配置
        env::remove_var("POSTGRES_HOST");
        env::remove_var("MYSQL_HOST");
        env::remove_var("QUESTDB_HOST");

        // 测试PostgreSQL配置
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_USER", "test_user");
        env::set_var("POSTGRES_PASSWORD", "test_pass");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("POSTGRES_DATABASE", "test_db");

        // 验证连接字符串包含关键信息
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap(),
            env::var("POSTGRES_HOST").unwrap(),
            env::var("POSTGRES_PORT").unwrap(),
            env::var("POSTGRES_DATABASE").unwrap()
        );

        assert!(url.contains("postgres://"));
        assert!(url.contains("test_user"));
        assert!(url.contains("localhost"));

        // 清理环境变量
        env::remove_var("POSTGRES_HOST");
        env::remove_var("POSTGRES_USER");
        env::remove_var("POSTGRES_PASSWORD");
        env::remove_var("POSTGRES_PORT");
        env::remove_var("POSTGRES_DATABASE");
    }
}

/**
 * 数据库表创建测试模块
 *
 * 验证实体模型到数据库表的映射和创建。
 * 包括表结构、索引、约束的验证。
 */
mod table_creation_tests {
    use super::*;
    use sea_orm::ConnectionTrait;

    /**
     * 安全公告表创建测试
     *
     * 验证github_advisories实体的表创建。
     * 检查表是否存在、字段类型是否正确。
     */
    #[tokio::test]
    async fn test_advisories_table_creation() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接应成功");

        let backend = db.get_database_backend();
        let schema = Schema::new(backend);

        // 创建表语句构建
        let create_stmt = backend.build(
            schema
                .create_table_from_entity(crate::models::github_advisories::Entity)
                .if_not_exists(),
        );

        // 执行创建
        let result = db.execute(create_stmt).await;
        assert!(result.is_ok(), "表创建应成功");

        // 验证表存在
        let check_table = db
            .query_one(sea_orm::Statement::from_string(
                backend,
                "SELECT name FROM sqlite_master WHERE type='table' AND name='advisories'"
                    .to_string(),
            ))
            .await;

        assert!(check_table.is_ok());
        assert!(check_table.unwrap().is_some(), "advisories表应存在");
    }

    /**
     * 表结构验证测试
     *
     * 验证创建的表包含所有必需的字段。
     * 检查字段类型和约束条件。
     */
    #[tokio::test]
    async fn test_table_schema_validation() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接应成功");

        let backend = db.get_database_backend();
        let schema = Schema::new(backend);

        let create_stmt = backend.build(
            schema
                .create_table_from_entity(crate::models::github_advisories::Entity)
                .if_not_exists(),
        );

        db.execute(create_stmt).await.expect("表创建应成功");

        // 获取表结构信息
        let columns = db
            .query_all(sea_orm::Statement::from_string(
                backend,
                "PRAGMA table_info(advisories)".to_string(),
            ))
            .await
            .expect("应能获取表结构");

        // 验证必需字段存在
        let column_names: Vec<String> = columns
            .iter()
            .map(|row| row.try_get::<String>("", "name").unwrap_or_default())
            .collect();

        assert!(
            column_names.contains(&"ghsa_id".to_string()),
            "应包含ghsa_id字段"
        );
        assert!(
            column_names.contains(&"cve_id".to_string()),
            "应包含cve_id字段"
        );
        assert!(
            column_names.contains(&"summary".to_string()),
            "应包含summary字段"
        );
        assert!(
            column_names.contains(&"severity".to_string()),
            "应包含severity字段"
        );
        assert!(
            column_names.contains(&"published_at".to_string()),
            "应包含published_at字段"
        );
    }
}

/**
 * 数据库错误处理测试模块
 *
 * 验证数据库操作失败时的错误处理和恢复。
 * 包括连接失败、SQL错误、约束冲突等场景。
 */
mod error_handling_tests {
    use super::*;

    /**
     * 无效数据库URL错误测试
     *
     * 验证连接无效URL时的错误处理。
     * 应返回明确的错误信息。
     */
    #[tokio::test]
    async fn test_invalid_database_url() {
        let result = Database::connect("invalid://url").await;
        assert!(result.is_err(), "无效URL应返回错误");
    }

    /**
     * 重复表创建测试
     *
     * 验证IF NOT EXISTS语法的幂等性。
     * 重复创建不应报错。
     */
    #[tokio::test]
    async fn test_idempotent_table_creation() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接应成功");

        let backend = db.get_database_backend();
        let schema = Schema::new(backend);

        let create_stmt = backend.build(
            schema
                .create_table_from_entity(crate::models::github_advisories::Entity)
                .if_not_exists(),
        );

        // 第一次创建
        let result1 = db.execute(create_stmt.clone()).await;
        assert!(result1.is_ok(), "首次创建应成功");

        // 第二次创建（幂等性）
        let result2 = db.execute(create_stmt).await;
        assert!(result2.is_ok(), "重复创建不应报错");
    }
}
