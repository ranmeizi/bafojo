use axum::{
    body::{Bytes, HttpBody},
    http::{ Request},
    middleware::{Next},
    response::{IntoResponse}
};
use chrono::prelude::Local;
use serde_json::{self, Value};

pub async fn json_timer<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    // 获取时间
    let now = Local::now().timestamp_millis();

    let res = next.run(req).await;

    let (mut parts, body) = res.into_parts();

    let body_text = body_into_text(body).await;

    match serde_json::from_str::<serde_json::Value>(&body_text.clone()) {
        Ok(mut json) => {
            let obj = json.as_object_mut().unwrap();

            // 添加花费字段
            obj.insert(
                String::from("cost"),
                Value::from(format!("{}ms", Local::now().timestamp_millis() - now)),
            );

            let body = serde_json::to_string(obj).unwrap();

            parts.headers.remove("content-length");

            (parts, body.into_response())
        }
        _ => (parts, body_text.into_response()),
    }
}

async fn body_into_text<B>(body: B) -> String
where
    B: HttpBody<Data = Bytes> + Unpin,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
