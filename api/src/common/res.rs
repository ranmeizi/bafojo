use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Res<T> {
    code: Option<i32>,
    data: Option<T>,
    msg: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    record: Vec<T>,
    total: usize,
    current: usize,
    page_size: usize,
}
