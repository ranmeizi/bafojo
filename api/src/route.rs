use axum::body::HttpBody;
use axum::{routing::get, Router};

// 创建路由的trait
pub trait CustCreate {
    fn with_route(self) -> Self;
}

impl<S, B> CustCreate for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn with_route(self) -> Self {
        self.route("/", get(|| async { "Hello, World!" }))
    }
}