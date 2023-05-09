use super::error::CustErrPairs;
use axum::{
    body::{self, Full},
    extract::rejection::{JsonRejection, QueryRejection},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

/**
 * 统一响应结构
 */
#[derive(Debug, Serialize)]
pub struct Res<T: Serialize = ()> {
    code: Option<u16>,
    data: Option<T>,
    msg: Option<String>,
}

impl<T: Serialize> Res<T> {
    /**
     * 普遍成功body
     */
    pub fn success(data: T) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            data: Some(data),
            msg: Some(String::from("success")),
        }
    }
}

impl Res<()> {
    /**
     * 普遍错误body
     */
    pub fn code_error(c: StatusCode) -> Self {
        Self {
            code: Some(c.as_u16()),
            data: None,
            msg: Some(String::from(c.canonical_reason().unwrap())),
        }
    }

    /**
     * 自定义错误body
     */
    pub fn cust_error(c: CustErrPairs) -> Self {
        Self {
            code: Some(c.0),
            data: None,
            msg: Some(String::from(c.1)),
        }
    }
}

/**
 * 解构 JsonRejection 时的响应结构
 */
impl From<JsonRejection> for Res<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: Some(value.status().as_u16()),
            data: None,
            msg: Some(value.body_text()),
        }
    }
}

/**
 * 解构 QueryRejection 时的响应结构
 */
impl From<QueryRejection> for Res<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: Some(value.status().as_u16()),
            data: None,
            msg: Some(value.body_text()),
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        let payload = json!(self);
        (StatusCode::FAILED_DEPENDENCY, Json(payload)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    record: Vec<T>,
    total: usize,
    current: usize,
    page_size: usize,
}
