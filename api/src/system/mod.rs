use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod resource;
mod role;
mod user;

pub fn system_routes() -> Router {
    Router::new().nest("/resource", resource_api())
}

pub fn resource_api() -> Router {
    Router::new()
        .route("/", get(resource::query))
        .route("/", post(resource::create))
        .route("/:id", get(resource::find_by_id))
        .route("/:id", post(resource::update))
        .route("/:id", patch(resource::update))
        .route("/:id", delete(resource::delete_by_id))
}
