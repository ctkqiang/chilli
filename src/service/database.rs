use crate::config::get_database_path;
use crate::models::github_advisories;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};

/**
 * 初始化数据库连接并创建必要的表结构
 *
 * # 功能描述
 * - 根据配置文件中的数据库路径建立SQLite连接
 * - 自动检测并创建必要的数据库表（如果不存在）
 * - 使用SeaORM ORM框架进行数据库操作
 *
 * # 参数说明
 * 此函数不接受任何参数，数据库路径从配置文件读取
 *
 * # 返回值
 * - 成功: 返回 `DatabaseConnection` 可用于后续数据库操作
 * - 失败: 返回 `DbErr` 错误信息，包含连接失败或表创建失败的原因
 *
 * # 使用示例
 * ```rust
 * let db = match database::initialise_db().await {
 *     Ok(connection) => connection,
 *     Err(e) => {
 *         eprintln!("数据库初始化失败: {}", e);
 *         process::exit(1);
 *     }
 * };
 * ```
 *
 * # 注意事项
 * - 首次运行时会自动创建 `./data/chilli.db` 文件
 * - 确保程序有写入数据目录的权限
 * - 表创建使用 `IF NOT EXISTS` 语法，重复运行不会报错
 */
pub async fn initialise_db() -> Result<DatabaseConnection, DbErr> {
    let sqlite_url = get_database_path();
    let db = Database::connect(&sqlite_url).await?;
    let db_backend = db.get_database_backend();

    let schema = Schema::new(db_backend);

    let create_table_op = db_backend.build(
        schema
            .create_table_from_entity(github_advisories::Entity)
            .if_not_exists(),
    );

    match db.execute(create_table_op).await {
        Ok(_) => Ok(db),
        Err(e) => Err(e),
    }
}
