use axum::{
    body::{self, Full},
    extract::rejection::{JsonRejection,QueryRejection},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct CommonRes<T: Serialize> {
    code: u16,
    data: Option<T>,
    msg: Option<String>,
}

// Manual impl
impl From<JsonRejection> for CommonRes<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: value.status().as_u16(),
            data: None,
            msg: Some(value.body_text()),
        }
    }
}
impl From<QueryRejection> for CommonRes<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: value.status().as_u16(),
            data: None,
            msg: Some(value.body_text()),
        }
    }
}

impl IntoResponse for CommonRes<()> {
    fn into_response(self) -> axum::response::Response {
        let payload = json!(self);
        (StatusCode::FAILED_DEPENDENCY, Json(payload)).into_response()
    }
}

/**
 * 自定义 Response trait
 */
trait CustResponse {
    /**
     * 返回带有data的json数据
     */
    fn success<T: Serialize>(self, data: T);
    /**
     * 返回带有error枚举类型的错误的json数据
     */
    fn error(self);
    /**
     * 设置header
     */
    fn set_header(self, key: &str, value: &str);
}

impl CustResponse for Response {
    fn success<T>(self, data: T) {}
    fn error(self) {}
    fn set_header(mut self, key: &str, value: &str) {
        // let headers = self.headers_mut();
        // headers.append(key, HeaderValue::from_static(value));
    }
}

pub enum Error {}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    record: Vec<T>,
    total: usize,
    current: usize,
    page_size: usize,
}
