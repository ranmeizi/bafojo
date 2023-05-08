use anyhow::Ok;
use axum::{
    extract::{Form, Json, Path, Query, State },
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_core::system::resource;
use serde::Deserialize;
use crate::common::res::CommonRes;

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
pub async fn query(WithRejection(pagination,_): WithRejection<Query<A>,CommonRes<()>>) -> impl IntoResponse {
    println!("list:{:?}", pagination);

    let body = String::from("hey");
    // String::from("query")
    // (
    //     StatusCode::CREATED,
    //     Ok(set_some_cookies),
    //     // append two `set-cookie` headers to the response
    //     // without overriding the ones added by `set_some_cookies`
    //     AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
    // )
    // AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")])
    let code: u16 = StatusCode::CREATED.into();
    println!("{}", code);
    (
        StatusCode::CREATED,
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        body,
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
pub async fn create(Json(params): Json<AddResourceParams>) -> String {
    String::from("create")
    // Res {
    //     code: Some(200),
    //     data: Some(String::from("value")),
    //     msg: Some(String::from("ss")),
    // }
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
