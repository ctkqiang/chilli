use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "security_issues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub severity: String,
    pub summary: String,
    pub description: Option<String>,
    pub ghsa_id: Option<String>,
    pub cve_id: Option<String>,
    pub package: String,
    pub current_version: String,
    pub vulnerable_range: Option<String>,
    pub fixed_version: Option<String>,
    pub published_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
