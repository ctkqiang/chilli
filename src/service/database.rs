use crate::config::get_database_path;
use crate::models::github_advisories;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};

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
