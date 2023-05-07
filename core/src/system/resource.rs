use bfj_db::entity::sys_resource;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

pub struct Query {}
pub struct Mutation {}

impl Query {
    pub async fn find_resource_by_id(db: &DatabaseConnection, id: i32) {}
}

impl Mutation {
    pub async fn create_resource(
        db: &DatabaseConnection,
        data: sys_resource::Model,
    ) -> Result<sys_resource::ActiveModel, DbErr> {
        sys_resource::ActiveModel {
            name: Set(data.name.to_owned()),
            r#type: Set(data.r#type.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
