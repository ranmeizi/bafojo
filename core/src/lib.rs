pub mod system;
pub use bfj_db::entity;
use serde::{Deserialize, Serialize};

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
