use axum::{
    body::{Bytes, HttpBody},
    http::Request,
    middleware::{AddExtension, Next},
    response::IntoResponse,
    Extension,
};
use bfj_common::dto::system::UserDto;
use chrono::prelude::Local;
use hyper::service::Service;
use serde_json::{self, Value};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthState {
    userinfo: String,
}

/**
 * jwt layer
 * - 验证 jwt
 * - 将用户信息写入 request
 */

pub async fn jwt_layer<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    // 校验
    // todo!();

    let state = Arc::new(AuthState {
        userinfo: "我是用户信息".into(),
    });

    // println!("sha ji ba ??,{:?}", a);

    req.extensions_mut().insert(Extension(state).0);

    // 添加 userinfo
    next.run(req).await
}
