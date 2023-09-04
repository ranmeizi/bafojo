use std::cell::RefCell;

use crate::entity::sys_resource;
use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use bfj_common::dto::system::{ResourceDto, ResourceNodeDto,UserDto};
use bfj_common::res::Res;
use bfj_common::{enums::system as EnumSys, error::CustErr};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
    Order
};
use serde::Deserialize;
use async_recursion::async_recursion;
use crate::util::mutation;

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
        id: i32,
    ) -> Result<Option<sys_resource::Model>> {
        let resource = sys_resource::Entity::find_by_id(id).one(db).await?;

        Ok(resource)
    }

    #[async_recursion]
    pub async fn find_children_by_code(
        db: &DatabaseConnection,
        code: String,
    ) -> Result<Vec<ResourceNodeDto>> {
        let list = sys_resource::Entity::find()
            .filter(sys_resource::Column::Parent.eq(code))
            .order_by(sys_resource::Column::Order, Order::Asc)
            .all(db)
            .await?;

        let mut children_list :Vec<ResourceNodeDto> = vec![];
    
        for model in list {
            let resource:ResourceDto = model.clone().into();
            let _code = model.code.clone();
            children_list.push(ResourceNodeDto{
                children: RefCell::new(Self::find_children_by_code(db, _code).await?),
                ..resource.into()
            })
        }

        Ok(children_list)
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

    /**
     * 是否存在关联子节点
     */
    pub async fn check_has_child(db: &DatabaseConnection, parent: &str) -> Result<bool> {
        let count = sys_resource::Entity::find()
            .filter(sys_resource::Column::Parent.eq(parent))
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
        userinfo: &Option<UserDto>,
    ) -> Result<sys_resource::Model> {
        // 判断 type 是否安全
        let r#type = match TryInto::<EnumSys::EnumResourceType>::try_into(params.r#type) {
            Ok(t) => t,
            Err(e) => {
                // 响应错误
                return Err(CustErr::ReqParamError("type 值超出枚举范围".to_owned()).into());
            }
        };

        let create_info = mutation::get_create_info(userinfo);

        // 判断 parent 是否存在，否则会产生脏数据
        if params.parent.ne("root") && !Query::check_unique_code(db, &params.parent).await? {
            // 响应错误
            return Err(CustErr::ReqParamError("不存在的parent".to_owned()).into());
        }

        let resource = sys_resource::ActiveModel {
            name: Set(params.name.to_owned()),
            r#type: Set(r#type.into()),
            code: Set(params.code),
            parent: Set(params.parent),
            title: Set(params.title),
            url: Set(params.url),
            desc: Set(params.desc),
            order: Set(Some(params.order.unwrap_or(0))),
            created_at: Set(create_info.created_at),
            created_by: Set(create_info.created_by),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(resource)
    }

    pub async fn update_resource(
        db: &DatabaseConnection,
        params: UpdateResourceParams,
        userinfo: &Option<UserDto>,
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

        let update_info = mutation::get_update_info(userinfo);

        // 更新修改时间
        resource.updated_at = Set(update_info.updated_at);
        resource.updated_by = Set(update_info.updated_by);

        let resource: sys_resource::Model = resource.update(db).await?;

        Ok(resource)
    }

    pub async fn delete_resource_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
        // 判断 id 是否存在并查询 code 值
        let resource = if let Some(x) = Query::find_resource_by_id(db, id).await? {
            x
        } else {
            return Err(
                CustErr::ReqDeleteFail(format!("不存在 id = {id} 的数据，请删除后重试")).into(),
            );
        };

        // 判断 是否有 parent 为 id 的数据，存在则不允许删除
        if Query::check_has_child(db, &resource.code).await? {
            // 响应错误
            return Err(CustErr::ReqDeleteFail(format!(
                "存在 parent = {} 的数据，请删除后重试",
                resource.code
            ))
            .into());
        }

        sys_resource::Entity::delete_by_id(id).exec(db).await?;

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
    id: i32,
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
