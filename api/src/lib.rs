use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, middleware, response::IntoResponse,
    BoxError, Extension, Router,
};
use bfj_common::{db_conn, dto::system::UserDto, res::Res, CFG, DB};
use bfj_middleware::{json_timer, status_json};
use route::api;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tower::ServiceBuilder;
mod auth;
mod route;
mod system;
use std::sync::Arc;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    let db = DB.get_or_init(db_conn).await.to_owned();
    let port = CFG.app.port.clone();
    let state = AppState { db };

    let boboan = Arc::new(BBa {
        name: "boboan".into(),
    });

    // build our application with a single route
    let app = Router::new()
        .nest("/", api())
        .with_state(state)
        .layer(middleware::from_fn(status_json::status_json_layer))
        .layer(middleware::from_fn(json_timer))
        .layer(Extension(boboan));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}",).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[derive(Clone, Debug)]
pub struct BBa {
    name: String,
}

pub type RouterType = Router<AppState>;
