use axum::{http::Request, middleware::Next, response::IntoResponse, Extension};
use bfj_common::{cache::user_info, dto::system::UserDto, get_db, res::Res, utils::jwt, AppState};
use bfj_core::system::user;
use hyper::{header::AUTHORIZATION, HeaderMap};
use sea_orm::DatabaseConnection;
use serde_json::{self, Value};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthState {
    userinfo: Option<UserDto>,
}

/**
 * jwt layer
 * - 验证 jwt
 * - 将用户信息写入 request
 */

pub async fn jwt_layer<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    // 校验
    // todo!();
    // 获取 header Authorization 中的 access_tokens

    let access_token = get_access_token(&req);

    // let state:AppState = req.into_parts();
    println!("{}", access_token);

    // 校验 token
    let userinfo = match jwt::check_access_token(&access_token) {
        Ok(claims) => {
            let uid = claims.uid;
            let db = get_db().await;
            get_user_info(db, uid).await
        }
        Err(e) => {
            // 错误时结束响应
            return Res::<()>::error(e).into_response();
        }
    };

    let state: Arc<AuthState> = Arc::new(AuthState { userinfo: userinfo });

    req.extensions_mut().insert(Extension(state).0);

    // 添加 userinfo
    next.run(req).await
}

/**
 * 先走缓存
 * 没有再走service
 */
// async fn append_userinfo()->UserDto {

// }

fn get_access_token<B>(req: &Request<B>) -> String {
    let headers = req.headers();

    match headers.get(AUTHORIZATION) {
        Some(value) => {
            let str = value.to_str().unwrap();

            if str.find("Bearer ").is_some() {
                String::from(&str[7..])
            } else {
                String::from("")
            }
        }
        _ => String::from(""),
    }
}

/**
 * 使用 uid 获取 userinfo
 * 优先使用缓存
 * 再调用service
 */
async fn get_user_info(db: &DatabaseConnection, uid: i32) -> Option<UserDto> {
    let uidstr = uid.to_string();
    

    if let Ok(uinfo) = user_info::get(&uidstr).await {
        // 使用缓存
        return Some(uinfo);
    } else {
        // 使用service
        if let Ok(uinfo) = user::Query::find_user_by_id(db, uid).await {
            // 存一下
            if uinfo.is_some(){
                user_info::set(&uidstr, uinfo.clone().unwrap());
            }
           
            uinfo
        } else {
            None
        }
    }
}
