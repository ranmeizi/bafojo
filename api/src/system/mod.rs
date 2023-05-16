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
        .route("/list", get(resource::query))
        .route("/findById", get(resource::find_by_id))
        .route("/create", post(resource::create))
        .route("/update", post(resource::update))
        .route("/deleteById", post(resource::delete_by_id))
}
