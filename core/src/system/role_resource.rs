use crate::{PageData, PageParams};
use anyhow::{anyhow, Ok, Result};
use bfj_common::{entity::sys_rel_role_resource, error::CustErr};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde::Deserialize;

pub struct Query {}
pub struct Mutation {}

impl Query {
    /**
     * 根据角色id数组查询资源id，按资源id去重
     */
    pub async fn get_by_ids(db: &DatabaseConnection, role_id: i32) -> Result<Vec<i32>> {
        let source_ids = sys_rel_role_resource::Entity::find()
            .filter(sys_rel_role_resource::Column::RoleId.eq(role_id))
            .group_by(sys_rel_role_resource::Column::ResourceId)
            .all(db)
            .await?;

        Ok(source_ids.iter().map(|x| x.resource_id).collect())
    }

    /**
     * 使用role/resource id 联合查询
     */
    pub async fn check_union(
        db: &DatabaseConnection,
        role_id: i32,
        resource_id: i32,
    ) -> Result<bool> {
        let count = sys_rel_role_resource::Entity::find()
            .filter(
                sys_rel_role_resource::Column::RoleId
                    .eq(role_id)
                    .add(sys_rel_role_resource::Column::ResourceId.eq(resource_id)),
            )
            .count(db)
            .await?;
        Ok(count > 0)
    }
}

impl Mutation {
    /**
     * 添加一个关系
     */
    pub async fn add_rel(db: &DatabaseConnection, role_id: i32, resource_id: i32) -> Result<()> {
        if Query::check_union(db, role_id, resource_id).await? {
            return Err(CustErr::AppRuleError("已存在的资源,不需要新增".to_owned()).into());
        }

        sys_rel_role_resource::ActiveModel {
            role_id: Set(role_id),
            resource_id: Set(resource_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    /**
     * 删除一个关系
     */
    pub async fn del_rel_by_id(
        db: &DatabaseConnection,
        role_id: i32,
        resource_id: i32,
    ) -> Result<()> {
        sys_rel_role_resource::Entity::delete_many()
            .filter(
                sys_rel_role_resource::Column::ResourceId
                    .eq(role_id)
                    .and(sys_rel_role_resource::Column::ResourceId.eq(resource_id)),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
