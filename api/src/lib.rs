use axum::Router;
use bfj_db::{db_conn, DB};
use route::CustCreate;
use sea_orm::DatabaseConnection;
use std::env;
mod route;
mod system;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    let db = DB.get_or_init(db_conn).await.to_owned();
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let state = AppState { db };

    // build our application with a single route
    let app = Router::new().with_route().with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}",).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}
