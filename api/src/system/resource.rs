use std::time::Duration;

use crate::common::error::ERR_TEST;
use crate::common::res::Res;
use anyhow::Ok;
use axum::{
    extract::{Form, Json, Path, Query, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_core::system::resource;
use serde::Deserialize;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
pub struct A {
    page_num: usize,
    page_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct B {
    name: Option<String>,
}

// 获取分页列表
pub async fn query(
    WithRejection(pagination, _): WithRejection<Query<A>, Res<()>>,
) -> impl IntoResponse {
    sleep(Duration::from_millis(1000)).await;
    (
        StatusCode::OK,
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        Res::success(String::from("success body")),
    )
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
pub async fn create(
    WithRejection(params, _): WithRejection<Json<AddResourceParams>, Res<()>>,
) -> impl IntoResponse {
    println!("api");
    (
        StatusCode::UNAUTHORIZED,
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        Res::code_error(StatusCode::UNAUTHORIZED),
    )
}

// 更新资源
pub async fn update() -> impl IntoResponse {
    (
        StatusCode::UNAUTHORIZED,
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        Res::cust_error(ERR_TEST),
    )
}

// 使用 id 获取资源
pub async fn find_by_id() -> String {
    String::from("find_by_id")
}

// 使用 id 删除资源
pub async fn delete_by_id() -> String {
    String::from("delete_by_id")
}
