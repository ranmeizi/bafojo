use thiserror::Error;

pub type CustErrPairs = (u16, &'static str);

#[derive(Error, Debug)]
pub enum CustErr {
    #[error("请求 参数错误")]
    ReqParamError
}