use crate::models::github_advisories::{ActiveModel, Column, Entity as Advisory, Model};
use reqwest::header::{ACCEPT, USER_AGENT};
use sea_orm::*;

pub async fn sync_github_advisories(
    db: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
}
