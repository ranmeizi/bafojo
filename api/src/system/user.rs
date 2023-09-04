use axum::{
    extract::{Extension, Json, Path, Query as ReqQuery, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_common::{dto::system::UserDto, res::Res,cache::user_info};
use bfj_core::{
    system::user::{self, Mutation, Query},
    PageParams,
};
use bfj_middleware::auth::AuthState;
use serde::Deserialize;
use std::sync::Arc;

use crate::{AppState};

#[derive(Deserialize)]
pub struct ByIdParams {
    id: i32,
}

// 获取分页列表
pub async fn query(
    state: State<AppState>,
    WithRejection(ReqQuery(page_params), _): WithRejection<ReqQuery<PageParams>, Res>,
    WithRejection(ReqQuery(params), _): WithRejection<ReqQuery<user::QueryUserListParams>, Res>,
) -> impl IntoResponse {
    let res = Query::get_user_list(&state.db, page_params, params).await;

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
    let res = Query::find_user_by_id(&state.db, id_params.id).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 创建资源
pub async fn create(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<user::AddUserParams>, Res>,
) -> impl IntoResponse {
    let res = Mutation::create_user(&state.db, params,&auth_state.userinfo).await;

    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 更新资源
pub async fn update(
    state: State<AppState>,
    Extension(auth_state): Extension<Arc<AuthState>>,
    WithRejection(Json(params), _): WithRejection<Json<user::UpdateUserParams>, Res>,
) -> impl IntoResponse {
    let id = params.id.clone();
    let res = Mutation::update_user(&state.db, params,&auth_state.userinfo).await;
    // 清除缓存
    user_info::del(&id.to_string());
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
    let id = id_params.id.clone();
    let res = Mutation::delete_user_by_id(&state.db, id_params.id).await;
    
    // 清除缓存
    user_info::del(&id.to_string());
    match res {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e),
    }
}

// 使用 id 启用用户
pub async fn enable_user(
    state: State<AppState>,
    WithRejection(Json(params), _): WithRejection<Json<user::EnabledParams>, Res>,
)->impl IntoResponse {
    let id = params.id.clone();
    let res = Mutation::user_enabled(&state.db, params).await;

     // 清除缓存
     user_info::del(&id.to_string());
     match res {
         Ok(data) => Res::success(data),
         Err(e) => Res::error(e),
     }
}