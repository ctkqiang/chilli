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

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRequest {
    pub package: String,
    pub version: String,
    pub ecosystem: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub severity: String,
    pub summary: String,
    pub ghsa_id: Option<String>,
    pub cve_id: Option<String>,
    pub vulnerable_range: Option<String>,
    pub patched_version: Option<String>,
}
