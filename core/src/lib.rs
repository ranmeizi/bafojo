pub mod system;
pub use bfj_db::entity;

pub struct PageParams {
    page_num: Option<usize>,
    page_size: Option<usize>,
}
