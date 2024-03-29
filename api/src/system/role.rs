use axum::{
    extract::{Json, Path, Query as ReqQuery, State,Extension},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_common::{res::Res,AppState};
use bfj_core::{
    system::role::{self, Mutation, Query},
    PageParams,
};
use serde::Deserialize;
use bfj_middleware::auth::AuthState;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ByIdParams {
    id: i32,
}

// 获取分页列表
pub async fn query(
    state: State<AppState>,
    WithRejection(ReqQuery(page_params), _): WithRejection<ReqQuery<PageParams>, Res>,
    WithRejection(ReqQuery(params), _): WithRejection<ReqQuery<role::QueryRoleParams>, Res>,
) -> impl IntoResponse {
    let res = Query::get_role_list(&state.db, page_params, params).await;

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
    let res = Query::find_role_by_id(&state.db, id_params.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 创建资源
pub async fn create(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<role::AddRoleParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::create_role(&state.db, params,&auth_state.userinfo).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 更新资源
pub async fn update(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<role::UpdateRoleParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::update_role(&state.db, params,&auth_state.userinfo).await;

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
    let res = Mutation::delete_role_by_id(&state.db, id_params.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}
