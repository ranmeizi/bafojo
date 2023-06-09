use axum::{
    extract::{Json, Path, Query as ReqQuery, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_common::res::Res;
use bfj_core::{
    // unknown::{self, Mutation, Query},
    PageParams,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct ByIdParams {
    id: i32,
}

// 获取分页列表
pub async fn query(
    state: State<AppState>,
    WithRejection(page_params, _): WithRejection<ReqQuery<PageParams>, Res>,
    WithRejection(params, _): WithRejection<ReqQuery<{unknown}>, Res>,
) -> impl IntoResponse {
    let res = Query::get_resource_list(&state.db, page_params.0, params.0).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::code_error_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

// 使用 id 获取资源
pub async fn find_by_id(
    state: State<AppState>,
    WithRejection(id_params, _): WithRejection<ReqQuery<ByIdParams>, Res>,
) -> impl IntoResponse {
    let res = Query::find_resource_by_id(&state.db, id_params.0.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::code_error_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

// 创建资源
pub async fn create(
    state: State<AppState>,
    WithRejection(params, _): WithRejection<Json<{unknown}>, Res>,
) -> impl IntoResponse {
    let res = Mutation::create_resource(&state.db, params.0).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 更新资源
pub async fn update(
    state: State<AppState>,
    WithRejection(params, _): WithRejection<Json<{unknown}>, Res>,
) -> impl IntoResponse {
    let res = Mutation::update_resource(&state.db, params.0).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 使用 id 删除资源
pub async fn delete_by_id(
    state: State<AppState>,
    WithRejection(id_params, _): WithRejection<Json<ByIdParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::delete_resource_by_id(&state.db, id_params.0.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}
