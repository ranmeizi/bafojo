use axum::{extract::Query, Json};

use bfj_core::system::resource;

// 获取分页列表
pub async fn query() -> String {
    String::from("query")
}

// 创建资源
pub async fn create() -> String {
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
