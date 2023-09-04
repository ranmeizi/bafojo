use axum::{
    extract::{Extension,Json, Path, Query as ReqQuery, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_common::{res::Res,AppState};
use bfj_core::{
    system::resource::{self, Mutation, Query},
    PageParams,
};
use hyper::Request;
use bfj_middleware::auth::AuthState;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ByIdParams {
    id: i32,
}


#[derive(Deserialize)]
pub struct ByCodeParams {
    code: String,
}

// 获取分页列表
pub async fn query(
    state: State<AppState>,
    WithRejection(ReqQuery(page_params), _): WithRejection<ReqQuery<PageParams>, Res>,
    WithRejection(ReqQuery(params), _): WithRejection<
        ReqQuery<resource::QueryResourceListParams>,
        Res,
    >,
) -> impl IntoResponse {
    let res = Query::get_resource_list(&state.db, page_params, params).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 使用 id 获取资源
pub async fn find_by_id(
    state: State<AppState>,
    WithRejection(ReqQuery(id_params), _): WithRejection<ReqQuery<ByIdParams>, Res>,
) -> impl IntoResponse {
    let res = Query::find_resource_by_id(&state.db, id_params.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 创建资源
pub async fn create(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<resource::AddResourceParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::create_resource(&state.db, params,&auth_state.userinfo).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 更新资源
pub async fn update(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<resource::UpdateResourceParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::update_resource(&state.db, params,&auth_state.userinfo).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 使用 id 删除资源
pub async fn delete_by_id(
    state: State<AppState>,
    WithRejection(Json(id_params), _): WithRejection<Json<ByIdParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::delete_resource_by_id(&state.db, id_params.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

pub async fn get_resource_tree(
    state: State<AppState>,
    WithRejection(Json(params), _): WithRejection<Json<ByCodeParams>, Res>,
) -> impl IntoResponse{
    let res = Query::find_children_by_code(&state.db, params.code).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}