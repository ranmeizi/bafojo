use axum::{middleware, Router};
use bfj_common::{db_conn, CFG, DB};
use bfj_middleware::json_timer;
use route::api;
use sea_orm::DatabaseConnection;
mod route;
mod system;
mod auth;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    let db = DB.get_or_init(db_conn).await.to_owned();
    let port = CFG.app.port.clone();
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
