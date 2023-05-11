use crate::RouterType;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod resource;
mod role;
mod user;

pub fn system_routes() -> RouterType {
    Router::new().nest("/resource", resource_api())
}

pub fn resource_api() -> RouterType {
    Router::new()
        .route("/", get(resource::query).post(resource::create))
        .route(
            "/:id",
            get(resource::find_by_id)
                .post(resource::update)
                .patch(resource::update)
                .delete(resource::delete_by_id),
        )
}
