use crate::config::get_database_path;
use rusqlite::Result;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn initialise_db() -> Result<DatabaseConnection, DbErr> {
    let sqlite_url = get_database_path();

    let db = Database::connect(&sqlite_url).await?;

    Ok(db)
}
