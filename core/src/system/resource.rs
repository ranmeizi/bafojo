use crate::entity::sys_resource;
use crate::{PageData, PageParams};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Select, Set,
};
use serde::Deserialize;

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
    pub async fn get_resource_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        params: QueryResourceListParams,
    ) -> Result<PageData<sys_resource::Model>, DbErr> {
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

        // 分页
        let total = s.clone().count(db).await?;
        // 分页获取数据
        let paginator = s
            .order_by_asc(sys_resource::Column::Id)
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
    ) -> Result<Option<sys_resource::Model>, DbErr> {
        sys_resource::Entity::find_by_id(id).one(db).await
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
 * 创建资源参数
 */
#[derive(Debug, Deserialize)]
pub struct AddResourceParams {
    name: String,
    r#type: String,
    code: String,
    pid: String,
    title: Option<String>,
    url: Option<String>,
    desc: Option<String>,
    order_id: Option<i8>,
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
