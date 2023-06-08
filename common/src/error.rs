use thiserror::Error;

pub type CustErrPairs = (u16, &'static str);

#[derive(Error, Debug)]
pub enum CustErr {
    #[error("unexpected error : {0}")]
    UnexpectedError(String),

    #[error("something wrong with Request parameter : {0}")]
    ReqParamError(String),

    #[error("something wrong with Request method : {0}")]
    ReqMethodErr(String),

    #[error("something wrong when Delete : {0}")]
    ReqDeleteFail(String),

    #[error("app rule : {0}")]
    AppRuleError(String),
}

#[derive(Error, Debug)]
pub enum AuthErr {
    #[error("auth error : 用户名或密码错误")]
    NoUser,

    #[error("auth error : 用户名或密码错误")]
    PasswordError,

    #[error("auth error : access_token 过期")]
    ExpiredToken,

    #[error("auth error : 无效的 access_token")]
    InvalidToken
}