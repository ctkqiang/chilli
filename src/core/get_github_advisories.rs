use crate::config::GITHUB_ADVISORIES_API_URL;
use crate::models::github_advisories::{ActiveModel, Column, Entity as Advisory, Model};
use crate::models::log_level::LogLevel;
use crate::utils;
use reqwest::header::{ACCEPT, USER_AGENT};
use sea_orm::*;

pub async fn sync_github_advisories(
    db: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let api_data = client
        .get(GITHUB_ADVISORIES_API_URL)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await?
        .json::<Vec<Model>>()
        .await?;

    let active_models: Vec<ActiveModel> = api_data
        .into_iter()
        .map(|m| m.into_active_model())
        .collect();

    let count = active_models.len();

    Advisory::insert_many(active_models)
        .on_conflict(
            sea_query::OnConflict::column(Column::GhsaId)
                .update_columns([Column::Summary, Column::Severity, Column::CveId])
                .to_owned(),
        )
        .exec(db)
        .await?;

    utils::logger::log(
        LogLevel::Debug,
        &format!("成功同步{}条GitHub安全公告", count),
    );

    Ok(())
}
