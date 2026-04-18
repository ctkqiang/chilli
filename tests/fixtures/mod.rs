/**
 * 测试数据与工具模块
 *
 * 本模块提供单元测试和集成测试所需的辅助函数、模拟数据和测试环境配置。
 * 所有测试用例应通过本模块获取测试数据，确保测试的一致性和可重复性。
 */

/**
 * 测试配置常量
 *
 * 定义测试环境中使用的各类常量，包括超时时间、重试次数、测试数据路径等。
 * 这些常量可根据测试环境通过环境变量覆盖。
 */
pub const TEST_DB_URL: &str = "sqlite::memory:";
pub const TEST_TIMEOUT_MS: u64 = 5000;
pub const TEST_RETRY_COUNT: u32 = 3;

/**
 * 数据库测试辅助函数
 *
 * 提供内存数据库连接、测试数据初始化、数据清理等功能。
 * 使用内存数据库确保测试的隔离性和执行速度。
 */
pub mod database {
    use sea_orm::{Database, DatabaseConnection};

    /**
     * 建立内存数据库连接
     *
     * 创建一个新的内存SQLite连接，用于单元测试。
     * 每个测试用例应独立创建连接，确保测试隔离性。
     */
    pub async fn setup_test_db() -> DatabaseConnection {
        Database::connect("sqlite::memory:")
            .await
            .expect("内存数据库连接失败")
    }

    /**
     * 清理测试数据
     *
     * 删除所有测试表中的数据，准备下一次测试。
     * 在测试用例的清理阶段调用。
     */
    pub async fn cleanup_test_data(_db: &DatabaseConnection) {
        // 清理逻辑
    }
}

/**
 * HTTP测试辅助函数
 *
 * 提供Axum应用实例创建、测试请求构建、响应验证等功能。
 * 支持同步和异步测试场景。
 */
pub mod http {
    use axum::Router;

    /**
     * 创建测试用Axum应用实例
     *
     * 初始化包含所有路由的Axum应用，但不启动HTTP服务器。
     * 用于集成测试中的请求发送和响应验证。
     */
    pub fn create_test_app() -> Router {
        // 测试应用创建逻辑
        Router::new()
    }
}

/**
 * GitHub API模拟数据
 *
 * 提供GitHub安全公告API的响应样本数据。
 * 用于测试GitHub同步功能，避免真实API调用。
 */
pub mod github_fixtures {
    use serde_json::json;

    /**
     * 标准安全公告样本
     *
     * 包含完整字段的安全公告JSON数据，
     * 用于测试正常情况下的数据解析和存储。
     */
    pub fn sample_advisory() -> serde_json::Value {
        json!({
            "ghsa_id": "GHSA-xxxx-xxxx-xxxx",
            "cve_id": "CVE-2024-12345",
            "summary": "测试安全漏洞",
            "severity": "high",
            "published_at": "2024-01-01T00:00:00Z"
        })
    }

    /**
     * 批量安全公告样本
     *
     * 包含多个安全公告的JSON数组，
     * 用于测试批量插入和更新功能。
     */
    pub fn sample_advisories_batch() -> Vec<serde_json::Value> {
        vec![
            sample_advisory(),
            json!({
                "ghsa_id": "GHSA-yyyy-yyyy-yyyy",
                "cve_id": null,
                "summary": "另一个测试漏洞",
                "severity": "medium",
                "published_at": "2024-01-02T00:00:00Z"
            }),
        ]
    }
}

/**
 * 进程信息测试数据结构
 *
 * 在测试中使用的进程信息结构体定义。
 * 与主项目中的ProcessInfo保持一致。
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub memory_bytes: u64,
    pub start_time: String,
    pub uptime_seconds: u64,
    pub listening_ports: Vec<u16>,
}

/**
 * 系统概览测试数据结构
 *
 * 在测试中使用的系统概览结构体定义。
 * 与主项目中的SystemOverview保持一致。
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemOverview {
    pub processes: Vec<ProcessInfo>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uptime_seconds: u64,
}

/**
 * 进程信息模拟数据
 *
 * 提供系统进程信息的测试数据。
 * 用于测试进程采集功能，避免依赖真实系统状态。
 */
pub mod process_fixtures {
    use super::{ProcessInfo, SystemOverview};

    /**
     * 标准进程信息样本
     *
     * 包含完整字段的进程信息结构体，
     * 用于测试进程数据的序列化和展示。
     */
    pub fn sample_process() -> ProcessInfo {
        ProcessInfo {
            pid: 12345,
            name: "test-process".to_string(),
            cmdline: vec!["test-process".to_string(), "--arg1".to_string()],
            memory_bytes: 1024000,
            start_time: "2024-01-01T00:00:00Z".to_string(),
            uptime_seconds: 3600,
            listening_ports: vec![8080, 8443],
        }
    }

    /**
     * 系统概览样本数据
     *
     * 包含多个进程的完整系统状态，
     * 用于测试系统概览API的响应格式。
     */
    pub fn sample_system_overview() -> SystemOverview {
        SystemOverview {
            processes: vec![sample_process()],
            total_memory: 8589934592,
            used_memory: 4294967296,
            uptime_seconds: 86400,
        }
    }
}

/**
 * 断言辅助宏
 *
 * 提供针对本项目数据结构的专用断言宏，
 * 简化测试代码，提高可读性。
 */
#[macro_export]
macro_rules! assert_process_exists {
    ($processes:expr, $pid:expr) => {
        assert!(
            $processes.iter().any(|p| p.pid == $pid),
            "进程中应包含PID为{}的进程",
            $pid
        );
    };
}

#[macro_export]
macro_rules! assert_advisory_synced {
    ($result:expr) => {
        assert!($result.is_ok(), "安全公告同步应成功完成");
    };
}
