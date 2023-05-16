use thiserror::Error;

pub type CustErrPairs = (u16, &'static str);

#[derive(Error, Debug)]
pub enum CustErr {
    #[error("something wrong with Request parameter : {0}")]
    ReqParamError(String),

    #[error("something wrong when Delete : {0}")]
    ReqDeleteFail(String)
}