use axum::{middleware, Extension, Router,http::{Method}};
use bfj_common::{get_db, AppState, CFG};
use bfj_middleware::{json_timer, status_json};
use route::api;
mod auth;
mod route;
mod system;
use tower_http::cors::{CorsLayer,Any};

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    let db = get_db().await;
    let port = CFG.app.port.clone();
    let state = AppState { db };

    let cors = CorsLayer::new()
    .allow_origin(Any) // 允许的来源
    .allow_methods(vec![Method::GET, Method::POST]); // 允许的 HTTP 方法

    // build our application with a single route
    let app = Router::new()
        .nest("/", api())
        .with_state(state)
        .layer(middleware::from_fn(status_json::status_json_layer))
        .layer(middleware::from_fn(json_timer))
        .layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}",).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub type RouterType = Router<AppState>;
