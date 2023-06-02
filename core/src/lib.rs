pub mod system;
pub mod auth;
pub mod util;
pub use bfj_common::entity;
use serde::{Deserialize, Serialize};

pub use util::crypto;

#[derive(Debug, Deserialize)]
pub struct PageParams {
    page_num: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageData<T> {
    record: Vec<T>,
    total: u64,
    current: u64,
    page_size: u64,
    total_pages: u64,
}
