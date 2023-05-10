use crate::system;
use axum::Router;
use crate::RouterType;

pub fn api() -> RouterType {
    Router::new().nest("/system", system::system_routes())
}
