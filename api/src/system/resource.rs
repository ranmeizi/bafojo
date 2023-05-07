use axum::extract::{Form, Json, Path, Query, State};
use serde::Deserialize;

use bfj_core::system::resource;

#[derive(Debug, Deserialize)]
pub struct A {
    page_num: usize,
    page_size: usize,
    name: Option<String>,
}

// 获取分页列表
pub async fn query(Query(query): Query<A>) -> String {
    println!("list:{:?}", query);
    String::from("query")
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

// 创建资源
pub async fn create(Json(params): Json<AddResourceParams>) -> String {
    String::from("create")
}

// 更新资源
pub async fn update() -> String {
    String::from("update")
}

// 使用 id 获取资源
pub async fn find_by_id() -> String {
    String::from("find_by_id")
}

// 使用 id 删除资源
pub async fn delete_by_id() -> String {
    String::from("delete_by_id")
}
