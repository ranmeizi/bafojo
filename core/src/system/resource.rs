use core::fmt;
use std::{format, println};

use crate::entity::sys_resource;
use crate::{PageData, PageParams};
use anyhow::{anyhow, Ok, Result};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Select, Set,
};
use serde::Deserialize;
use bfj_common::error::CustErr;

enum ResourceType {
    Permission,
    PermissionMenu,
}

pub struct Query {}
pub struct Mutation {}

/**
 * 查询
 */
impl Query {
    /**
     * 获取列表
     */
    pub async fn get_resource_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        params: QueryResourceListParams,
    ) -> Result<PageData<sys_resource::Model>> {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(20);

        // 组装查询条件
        let mut s = sys_resource::Entity::find();

        if let Some(search_text) = params.search {
            s = s.filter(
                sys_resource::Column::Name
                    .like(&format!("%{}%", search_text))
                    .or(sys_resource::Column::Code.like(&format!("%{}%", search_text)))
                    .or(sys_resource::Column::Title.like(&format!("%{}%", search_text))),
            );
        };

        if let Some(r#type) = params.r#type {
            s = s.filter(sys_resource::Column::Type.eq(r#type))
        }

        // 分页
        let total = s.clone().count(db).await?;
        let paginator = s
            .order_by_asc(sys_resource::Column::CreatedAt)
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
    pub async fn find_resource_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<sys_resource::Model>> {
        let resource = sys_resource::Entity::find_by_id(id).one(db).await?;

        Ok(resource)
    }

    /**
     * 是否存在重复code?
     */
    pub async fn check_unique_code(db: &DatabaseConnection, code: &str) -> Result<bool> {
        let count = sys_resource::Entity::find()
            .filter(sys_resource::Column::Code.eq(code))
            .count(db)
            .await?;
        Ok(count > 0)
    }
}

/**
 * 修改
 */
impl Mutation {
    /**
     * 创建资源
     */
    pub async fn create_resource(
        db: &DatabaseConnection,
        params: AddResourceParams,
    ) -> Result<sys_resource::Model> {
        // 判断 parent 是否存在，否则会产生脏数据
        println!("??:{}", params.parent.ne("root"));
        if params.parent.ne("root") && !Query::check_unique_code(db, &params.parent).await? {
            // 响应错误
            return Err(CustErr::ReqParamError.into());
        }

        let resource = sys_resource::ActiveModel {
            name: Set(params.name.to_owned()),
            r#type: Set(params.r#type.to_owned()),
            created_at: Set(Some(Utc::now())),
            code: Set(params.code),
            parent: Set(params.parent),
            title: Set(params.title),
            url: Set(params.url),
            desc: Set(params.desc),
            order: Set(Some(params.order.unwrap_or_default())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(resource)
    }

    pub async fn update_resource(
        db: &DatabaseConnection,
        params: UpdateResourceParams,
    ) -> Result<sys_resource::Model> {
        let resource: Option<sys_resource::Model> =
            sys_resource::Entity::find_by_id(params.id).one(db).await?;

        // Into ActiveModel
        let mut resource: sys_resource::ActiveModel = resource.unwrap().into();

        if let Some(name) = params.name {
            resource.name = Set(name);
        }

        if let Some(title) = params.title {
            resource.title = Set(Some(title));
        }

        if let Some(url) = params.url {
            resource.url = Set(Some(url));
        }

        if let Some(desc) = params.desc {
            resource.desc = Set(Some(desc));
        }

        if let Some(order) = params.order {
            resource.order = Set(Some(order));
        }

        let resource: sys_resource::Model = resource.update(db).await?;

        Ok(resource)
    }

    pub async fn delete_resource_by_id(db: &DatabaseConnection, id: String) -> Result<()> {
        // TODO 删除前检查是否有对应parentid

        sys_resource::Entity::delete_by_id(&id).exec(db).await?;

        Ok(())
    }
}

/**
 * 创建资源参数
 */
#[derive(Debug, Deserialize)]
pub struct AddResourceParams {
    name: String,
    r#type: String,
    code: String,
    parent: String,
    title: Option<String>,
    url: Option<String>,
    desc: Option<String>,
    order: Option<i16>,
}

/**
 * 更新资源参数
 */
#[derive(Debug, Deserialize)]
pub struct UpdateResourceParams {
    id: String,
    name: Option<String>,
    title: Option<String>,
    url: Option<String>,
    desc: Option<String>,
    order: Option<i16>,
}

/**
 * 资源筛选参数
 */
#[derive(Debug, Deserialize)]
pub struct QueryResourceListParams {
    // 查询字符串  模糊查询  name/code/title
    search: Option<String>,
    r#type: Option<String>,
}

pub struct UniqueColumn {
    code: Option<String>,
}
