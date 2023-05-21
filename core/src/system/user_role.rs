use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use bfj_common::{entity::sys_rel_user_role, error::CustErr};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;

pub struct Query {}
pub struct Mutation {}

impl Query {
    /**
     * 根据用户id查询角色
     */
    pub async fn get_all_ids_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<i32>> {
        let source_ids = sys_rel_user_role::Entity::find()
            .filter(sys_rel_user_role::Column::RoleId.eq(user_id))
            .all(db)
            .await?;

        Ok(source_ids.iter().map(|x| x.role_id).collect())
    }
}

impl Mutation {
    pub fn add_rel() {}

    pub fn del_rel_by_id() {}
}
