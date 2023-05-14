use super::error::{CustErr, CustErrPairs};
use axum::{
    body::{self, Full},
    extract::rejection::{JsonRejection, QueryRejection},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fmt::Debug, println};
// use std::error::Error;
use anyhow::Error;

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

    /**
     * 普遍成功body 带有msg
     */
    pub fn success_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            data: Some(data),
            msg: Some(String::from(msg)),
        }
    }
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
     * 普遍错误body
     */
    pub fn code_error_msg(c: StatusCode, msg: &str) -> Self {
        Self {
            code: Some(c.as_u16()),
            data: None,
            msg: Some(String::from(msg)),
        }
    }
    /**
     * 自定义错误body
     */
    pub fn cust_error(e: Error) -> Self {
        println!("SOURCE {:?}",e.downcast_ref::<CustErr>());
        Self {
            code: Some(400),
            data: None,
            msg: Some(e.to_string()),
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
        (StatusCode::OK, Json(payload)).into_response()
    }
}
