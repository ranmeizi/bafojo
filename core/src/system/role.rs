use crate::entity::sys_role;
use crate::{PageData, PageParams};
use crate::util::mutation;
use anyhow::{anyhow, Result};
use bfj_common::{dto::system::{RoleDto,UserDto}, error::CustErr};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

pub struct Query {}
pub struct Mutation {}

impl Query {
    /**
     * 获取列表
     */
    pub async fn get_role_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        params: QueryRoleParams,
    ) -> Result<PageData<RoleDto>> {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(20);

        // 组装查询条件
        let mut s = sys_role::Entity::find();

        if let Some(name) = params.name {
            s = s.filter(sys_role::Column::Name.like(&format!("%{name}%")));
        }

        // 分页
        let total = s.clone().count(db).await?;
        let paginator = s
            .order_by_asc(sys_role::Column::CreatedAt)
            .paginate(db, page_size);
        let total_pages = paginator.num_pages().await?;
        let list: Vec<RoleDto> = paginator
            .fetch_page(page_num - 1)
            .await?
            .iter()
            .map(|m| m.clone().into())
            .collect();

        Ok(PageData {
            record: list,
            total: total,
            current: page_num,
            page_size: page_size,
            total_pages: total_pages,
        })
    }

    /**
     * 按id查询
     */
    pub async fn find_role_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<sys_role::Model>> {
        let role = sys_role::Entity::find_by_id(id).one(db).await?;

        Ok(role)
    }

    pub async fn check_unique_name(db: &DatabaseConnection, uname: &str) -> Result<bool> {
        let count = sys_role::Entity::find()
            .filter(sys_role::Column::Name.eq(uname))
            .count(db)
            .await?;
        Ok(count > 0)
    }
}

impl Mutation {
    /**
     * 创建
     */
    pub async fn create_role(
        db: &DatabaseConnection,
        params: AddRoleParams,
        userinfo: &Option<UserDto>,
    ) -> Result<RoleDto> {
        // 判断 uname 是否重复
        if Query::check_unique_name(db, &params.name).await? {
            // 响应错误
            return Err(CustErr::AppRuleError("用户名不可以重复".to_owned()).into());
        }

        let create_info = mutation::get_create_info(userinfo);

        let role = sys_role::ActiveModel {
            name: Set(params.name),
            desc: Set(params.desc),
            created_at: Set(create_info.created_at),
            created_by: Set(create_info.created_by),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(role.into())
    }

    /**
     * 更新
     */
    pub async fn update_role(
        db: &DatabaseConnection,
        params: UpdateRoleParams,
        userinfo: &Option<UserDto>,
    ) -> Result<RoleDto> {
        let role: Option<sys_role::Model> = sys_role::Entity::find_by_id(params.id).one(db).await?;

        // Into ActiveModel
        let mut role: sys_role::ActiveModel = role.unwrap().into();

        role.name = Set(params.name);
        role.desc = Set(params.desc);

        let update_info = mutation::get_update_info(userinfo);

        // 更新修改时间
        role.updated_at = Set(update_info.updated_at);
        role.updated_by = Set(update_info.updated_by);

        let role: sys_role::Model = role.update(db).await?;

        Ok(role.into())
    }

    /**
     * 删除
     */
    pub async fn delete_role_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
        sys_role::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

/// QueryRoleParams
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleParams {
    /// 角色名称
    name: Option<String>,
}

/// AddRoleParams
#[derive(Debug, Deserialize)]
pub struct AddRoleParams {
    /// 角色描述
    desc: Option<String>,

    /// 角色名称
    name: String,
}

/// UpdateRoleParams
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleParams {
    /// 角色描述
    desc: Option<String>,

    /// 角色id
    id: i32,

    /// 角色名称
    name: String,
}
