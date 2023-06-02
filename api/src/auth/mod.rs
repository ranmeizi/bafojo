use crate::RouterType;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod auth;

pub fn auth_api() -> RouterType {
    Router::new().route("/login", post(auth::login))
}
