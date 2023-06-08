use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use bfj_common::res::Res;
use hyper::StatusCode;
use serde::Serialize;

struct CacheConfig<'a> {
    // 前缀
    prefix: &'a str,
    key: &'a str,
}

/**
 * 处理非 200 错误
 */

pub fn wrap_cache_config(prefix: &str) {
    async fn id_cache<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
        // 获取id参数

        let (parts, body) = next.run(req).await.into_parts();

        (parts, body.into_response())
    }
}
