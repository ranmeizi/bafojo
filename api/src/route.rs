use crate::system;
use axum::Router;

pub fn api() -> Router {
    Router::new().nest("/system", system::system_routes())
}
