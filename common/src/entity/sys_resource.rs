//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_resource")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub code: String,
    pub parent: String,
    pub name: String,
    pub r#type: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub order: Option<i16>,
    pub enabled: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub created_by: Option<i32>,
    pub updated_at: Option<DateTimeUtc>,
    pub updated_by: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
