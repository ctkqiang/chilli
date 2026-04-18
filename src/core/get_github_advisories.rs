use crate::config::GITHUB_ADVISORIES_API_URL;
use crate::models::github_advisories::{ActiveModel, Column, Entity as Advisory, Model};
use crate::models::log_level::LogLevel;
use crate::utils;
use reqwest::header::{ACCEPT, USER_AGENT};
use sea_orm::*;

/**
 * 从GitHub API同步安全公告数据到本地数据库
 *
 * # 功能概述
 * 本函数负责从小辣椒服务的GitHub安全公告API获取最新的安全漏洞信息，
 * 并将其批量插入到本地数据库中。采用"插入或更新"策略，确保数据始终保持最新。
 *
 * # 数据流架构
 * ```
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                      GitHub安全公告同步流程                              │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │   ┌─────────────────┐                                                   │
 * │   │ GitHub API      │                                                   │
 * │   │ /advisories     │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼ HTTP GET + Headers                                          │
 * │   ┌─────────────────┐                                                   │
 * │   │ reqwest::Client │                                                   │
 * │   │ 发送API请求      │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼ JSON解析                                                   │
 * │   ┌─────────────────┐                                                   │
 * │   │ Vec<Model>      │                                                   │
 * │   │ API响应数据      │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼ 转换为ActiveModel                                           │
 * │   ┌─────────────────┐                                                   │
 * │   │ Vec<ActiveModel>│                                                   │
 * │   │ 数据库操作模型   │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼ 批量插入/更新                                               │
 * │   ┌─────────────────┐     ┌─────────────────┐                          │
 * │   │ INSERT INTO     │────▶│ ON CONFLICT     │                          │
 * │   │ advisories      │     │ UPDATE SET      │                          │
 * │   └────────┬────────┘     └─────────────────┘                          │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │ SQLite/MySQL/   │                                                   │
 * │   │ PostgreSQL      │                                                   │
 * │   └─────────────────┘                                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 冲突解决策略
 * 当数据库中已存在相同 `ghsa_id` 的记录时，自动更新以下字段：
 * - `summary`: 漏洞摘要描述
 * - `severity`: 严重程度
 * - `cve_id`: CVE编号
 *
 * 这种设计确保：
 * 1. 新公告会被插入
 * 2. 已有公告的信息会被更新
 * 3. 不会创建重复记录
 *
 * # API请求头
 * - `User-Agent`: 标识客户端身份
 * - `Accept`: 指定接收GitHub JSON格式响应
 *
 * # 参数
 * - `db`: 数据库连接引用，用于执行批量插入操作
 *
 * # 返回值
 * - `Ok(())`: 同步成功
 * - `Err(Box<dyn std::error::Error>)`: 可能的错误包括：
 *   - 网络请求失败（连接超时、DNS错误等）
 *   - API返回非200状态码
 *   - JSON解析失败
 *   - 数据库连接失败
 *   - SQL执行失败
 *
 * # 使用示例
 * ```rust
 * use sea_orm::Database;
 *
 * #[tokio::main]
 * async fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let db = Database::connect("sqlite://./data.db").await?;
 *
 *     match sync_github_advisories(&db).await {
 *         Ok(_) => println!("安全公告同步成功！"),
 *         Err(e) => eprintln!("同步失败: {}", e),
 *     }
 *
 *     Ok(())
 * }
 * ```
 *
 * # 性能考虑
 * - 使用 `insert_many` 进行批量插入，比单条插入效率高
 * - 一次性获取所有数据后再批量处理，减少API调用次数
 * - 冲突检测在数据库层面完成，避免应用层查询
 *
 * # 注意事项
 * - GitHub API有速率限制，频繁调用可能触发限制
 * - 建议在后台任务中定期执行（如每天一次）
 * - 生产环境应考虑添加重试机制和错误告警
 * - 大量数据同步时可能占用较多内存
 */
pub async fn sync_github_advisories(
    db: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let api_data = client
        .get(GITHUB_ADVISORIES_API_URL)
        .header(USER_AGENT, "chilli-security-service")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await?
        .json::<Vec<Model>>()
        .await?;

    let count = api_data.len();
    let active_models: Vec<ActiveModel> = api_data
        .into_iter()
        .map(|m| m.into_active_model())
        .collect();

    Advisory::insert_many(active_models)
        .on_conflict(
            sea_query::OnConflict::column(Column::GhsaId)
                .update_columns([Column::Summary, Column::Severity, Column::CveId])
                .to_owned(),
        )
        .exec(db)
        .await?;

    utils::logger::log(
        LogLevel::Info,
        &format!("GitHub安全公告同步完成: {}条记录", count),
    );

    Ok(())
}
