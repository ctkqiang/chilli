use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "advisories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub summary: String,
    pub severity: String,
    pub published_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
