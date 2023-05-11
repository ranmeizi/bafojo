use axum::{
    extract::{Json, Path, Query as ReqQuery, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_core::{
    entity::sys_resource,
    system::resource::{self, Mutation, Query},
    PageParams,
};
use sea_orm::TryIntoModel;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

use crate::common::res::Res;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct B {
    name: Option<String>,
}

// 获取分页列表
pub async fn query(
    state: State<AppState>,
    WithRejection(page_params, _): WithRejection<ReqQuery<PageParams>, Res>,
    WithRejection(params, _): WithRejection<ReqQuery<resource::QueryResourceListParams>, Res>,
) -> impl IntoResponse {
    let res = Query::get_resource_list(&state.db, page_params.0, params.0).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::code_error_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

// 创建资源
pub async fn create(
    state: State<AppState>,
    WithRejection(params, _): WithRejection<Json<resource::AddResourceParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::create_resource(&state.db, params.0).await;

    match res {
        Ok(data) => Res::success(data.try_into_model().unwrap()),
        Err(e) => Res::code_error_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

// 更新资源
pub async fn update() -> impl IntoResponse {
    (
        StatusCode::UNAUTHORIZED,
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        Res::success(1),
    )
}

// 使用 id 获取资源
pub async fn find_by_id(state: State<AppState>, id: Path<i32>) -> impl IntoResponse {
    let res = Query::find_resource_by_id(&state.db, *id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::code_error_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

// 使用 id 删除资源
pub async fn delete_by_id() -> String {
    String::from("delete_by_id")
}
