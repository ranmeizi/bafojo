use crate::entity::sys_resource;
use chrono::prelude::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set, EntityTrait};
use serde::Deserialize;

enum ResourceType {
    Permission,
    PermissionMenu,
}

pub struct Query {}
pub struct Mutation {}

impl Query {
    pub async fn find_resource_by_id(db: &DatabaseConnection, id: i32)->Result<Option<sys_resource::Model>,DbErr> {
         sys_resource::Entity::find_by_id(id).one(db).await
    }
}

impl Mutation {
    pub async fn create_resource(
        db: &DatabaseConnection,
        params: AddResourceParams,
    ) -> Result<sys_resource::ActiveModel, DbErr> {
        sys_resource::ActiveModel {
            name: Set(params.name.to_owned()),
            r#type: Set(params.r#type.to_owned()),
            created_at: Set(Some(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }
}

/**
 * 创建资源的入参
 */
#[derive(Debug, Deserialize)]
pub struct AddResourceParams {
    name: String,
    r#type: String,
    url: Option<String>,
    desc: Option<String>,
    order_id: Option<i8>,
}
