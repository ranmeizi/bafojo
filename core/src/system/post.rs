use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde::Deserialize;

pub struct Query {}
pub struct Mutation {}

impl Query {
    /**
     * 获取列表
     */
    pub async fn get_post_list(db: &DatabaseConnection, page_params: PageParams)
    // -> Result<PageData<>>
    {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(20);

        // // 组装查询条件
        // let mut s = ::Entity::find();

        // // 分页
        // let total = s.clone().count(db).await?;
        // let paginator = s
        //     .order_by_asc(::Column::CreatedAt)
        //     .paginate(db, page_size);
        // let total_pages = paginator.num_pages().await?;
        // let list = paginator.fetch_page(page_num - 1).await?;

        // Ok(PageData {
        //     record: list,
        //     total: total,
        //     current: page_num,
        //     page_size: page_size,
        //     total_pages: total_pages,
        // })
    }

    /**
     * 按id查询
     */
    pub async fn find_post_by_id(db: &DatabaseConnection, id: i32) {}
}

impl Mutation {
    /**
     * 创建
     */
    pub async fn create_post(db: &DatabaseConnection) {}

    /**
     * 更新
     */
    pub async fn update_post(db: &DatabaseConnection) {}

    /**
     * 删除
     */
    pub async fn delete_post_by_id(db: &DatabaseConnection, id: i32) {}
}
