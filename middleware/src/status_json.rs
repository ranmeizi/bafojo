use axum::{http::Request, middleware::Next, response::IntoResponse};
use bfj_common::res::Res;
use hyper::StatusCode;

/**
 * 处理非 200 错误
 */

pub async fn status_json_layer<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let (parts, body) = next.run(req).await.into_parts();

    let code = parts.status;

    if code != StatusCode::OK {
        (
            parts,
            Res::<()> {
                code: Some(code.as_u16()),
                data: None,
                msg: Some(String::from(code.canonical_reason().unwrap())),
            }
            .into_response(),
        )
    } else {
        (parts, body.into_response())
    }
}
