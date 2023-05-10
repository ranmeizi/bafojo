use axum::{middleware, Router};
use bfj_db::{db_conn, DB};
use bfj_middleware::json_timer;
use route::api;
use sea_orm::DatabaseConnection;
use std::env;
mod common;
mod route;
mod system;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    let db = DB.get_or_init(db_conn).await.to_owned();
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let state = AppState { db };

    // build our application with a single route
    let app = Router::new()
        .nest("/", api())
        .with_state(state)
        .layer(middleware::from_fn(json_timer));

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

pub type RouterType = Router<AppState>;