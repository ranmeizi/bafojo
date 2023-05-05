use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"),
    );
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await.expect("数据库连接失败");
    db
}
