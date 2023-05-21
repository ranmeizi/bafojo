use crate::entity::sys_role;
use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use bfj_common::{dto::system::UserDto, error::CustErr};
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
     * 获取列表
     */
    pub async fn get_role_list(
        db: &DatabaseConnection,
        page_params: PageParams,
    ) -> Result<PageData<sys_role::Model>> {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(20);

        // 组装查询条件
        let mut s = sys_role::Entity::find();

        // 分页
        let total = s.clone().count(db).await?;
        let paginator = s
            .order_by_asc(sys_role::Column::CreatedAt)
            .paginate(db, page_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;

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
    ) -> Result<sys_role::Model> {
        // 判断 uname 是否重复
        if Query::check_unique_name(db, &params.name).await? {
            // 响应错误
            return Err(CustErr::AppRuleError("用户名不可以重复".to_owned()).into());
        }

        let role = sys_role::ActiveModel {
            name: Set(params.name),
            desc: Set(params.desc),
            created_at: Set(Some(Utc::now())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(role)
    }

    /**
     * 更新
     */
    pub async fn update_role(
        db: &DatabaseConnection,
        params: UpdateRoleParams,
    ) -> Result<sys_role::Model> {
        let role: Option<sys_role::Model> = sys_role::Entity::find_by_id(params.id).one(db).await?;

        // Into ActiveModel
        let mut role: sys_role::ActiveModel = role.unwrap().into();

        role.desc = Set(Some(params.desc));

        // 更新修改时间
        role.updated_at = Set(Some(Utc::now()));

        let role: sys_role::Model = role.update(db).await?;

        Ok(role)
    }

    /**
     * 删除
     */
    pub async fn delete_role_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
        sys_role::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

/**
 * 创建角色参数
 */
#[derive(Debug, Deserialize)]
pub struct AddRoleParams {
    name: String,
    desc: Option<String>,
    resourceIds: Vec<String>, // 资源id
}

/**
 * 创建角色参数
 */
#[derive(Debug, Deserialize)]
pub struct UpdateRoleParams {
    id: i32,
    desc: String,
    resourceIds: Vec<String>, // 资源id
}
