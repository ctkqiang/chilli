use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "access_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub src_ip: String,
    pub dst_port: i32,
    pub process_name: String,
    pub pid: i64,
    pub timestamp: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
