use crate::RouterType;
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use bfj_middleware::auth::jwt_layer;

mod resource;
mod role;
mod user;

pub fn system_routes() -> RouterType {
    Router::new()
        .nest(
            "/resource",
            resource_api().layer(middleware::from_fn(jwt_layer)),
        )
        .nest("/user", user_api().layer(middleware::from_fn(jwt_layer)))
}

pub fn resource_api() -> RouterType {
    Router::new()
        .route("/list", get(resource::query))
        .route("/findById", get(resource::find_by_id))
        .route("/create", post(resource::create))
        .route("/update", post(resource::update))
        .route("/deleteById", post(resource::delete_by_id))
        .route("/tree", post(resource::get_resource_tree))
}

pub fn user_api() -> RouterType {
    Router::new()
        .route("/list", get(user::query))
        .route("/findById", get(user::find_by_id))
        .route("/create", post(user::create))
        .route("/update", post(user::update))
        .route("/deleteById", post(user::delete_by_id))
        .route("/enabledUser",post(user::enable_user))
}
