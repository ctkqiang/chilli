use crate::config::LOCAL_DATABASE_PATH;
use rusqlite::Result;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn initialise_db() -> Result<DatabaseConnection, DbErr> {
    let sqlite_url = format!("sqlite:{}", LOCAL_DATABASE_PATH);

    let db = Database::connect(sqlite_url).await?;

    Ok(db)
}
