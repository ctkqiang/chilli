/**
 * 小辣椒测试套件主模块
 *
 * 本模块是测试套件的入口点，组织和协调所有测试模块的执行。
 * 测试分为单元测试、集成测试和性能测试三个层次。
 *
 * # 测试架构
 *
 * ```
 * tests/
 * ├── mod.rs                    # 测试主入口（本文件）
 * ├── fixtures/                 # 测试数据和工具
 * │   └── mod.rs               # 测试辅助函数和模拟数据
 * ├── unit/                    # 单元测试
 * │   ├── database_test.rs    # 数据库操作测试
 * │   ├── core_test.rs        # 核心业务逻辑测试
 * │   └── routes_test.rs      # HTTP路由测试
 * └── integration/            # 集成测试
 *     └── api_integration_test.rs  # API端到端测试
 * ```
 *
 * # 运行测试
 *
 * ## 运行所有测试
 * ```bash
 * cargo test
 * ```
 *
 * ## 运行特定模块测试
 * ```bash
 * cargo test unit::database_tests
 * cargo test integration::api_tests
 * ```
 *
 * ## 运行并显示输出
 * ```bash
 * cargo test -- --nocapture
 * ```
 *
 * ## 运行性能测试
 * ```bash
 * cargo test --release performance::
 * ```
 *
 * # 测试环境要求
 *
 * - SQLite内存数据库（自动创建，无需配置）
 * - 网络访问（用于GitHub API集成测试，可选）
 * - 足够的系统权限（用于进程管理测试）
 *
 * # 测试数据管理
 *
 * 所有测试使用fixtures模块提供的模拟数据，确保：
 * - 测试的可重复性
 * - 数据的一致性
 * - 执行的隔离性
 */

/**
 * 测试数据与工具模块
 *
 * 提供测试所需的辅助函数、模拟数据和共享工具。
 */
pub mod fixtures;

/**
 * 单元测试模块
 *
 * 测试单个组件的功能，不依赖外部服务。
 * 每个测试用例独立执行，完全隔离。
 */
pub mod unit {
    /**
     * 数据库操作单元测试
     *
     * 测试数据库连接、表创建、CRUD操作、事务处理。
     * 使用内存SQLite确保测试速度和隔离性。
     */
    pub mod database_test;

    /**
     * 核心业务逻辑单元测试
     *
     * 测试GitHub同步、进程采集、数据处理等核心功能。
     * 使用模拟数据避免外部依赖。
     */
    pub mod core_test;

    /**
     * HTTP路由单元测试
     *
     * 测试Axum路由的响应格式、状态码、错误处理。
     * 使用tower::ServiceExt进行请求测试。
     */
    pub mod routes_test;
}

/**
 * 集成测试模块
 *
 * 测试多个组件的协同工作，验证完整的业务流程。
 * 使用真实或模拟的外部依赖。
 */
pub mod integration {
    /**
     * API集成测试
     *
     * 测试完整的API调用链路，从HTTP请求到数据库操作。
     * 验证系统的端到端功能。
     */
    pub mod api_integration_test;
}

/**
 * 共享测试配置
 *
 * 定义所有测试模块共享的常量和类型。
 */
pub const TEST_VERSION: &str = "1.0.0";
