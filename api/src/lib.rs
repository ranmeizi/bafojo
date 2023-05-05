use axum::{
    routing::get,
    Router,
};
use bfj_db::{db_conn, DB};
use sea_orm::DatabaseConnection;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    // build our application with a single route
    let app = Router::new().route("/", get(hello::index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}
