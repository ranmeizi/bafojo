use crate::RouterType;
use crate::{auth, system};
use axum::Router;

pub fn api() -> RouterType {
    Router::new()
        .nest("/system", system::system_routes())
        .nest("/auth", auth::auth_api())
}
