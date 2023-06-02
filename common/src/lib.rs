/*
 * @Author: boboan 360969885@qq.com
 * @Date: 2023-05-30 13:41:59
 * @LastEditors: boboan 360969885@qq.com
 * @LastEditTime: 2023-05-31 23:45:08
 * @FilePath: /bafojo/common/src/lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;
mod config;
pub use config::CFG;
pub mod res;
pub mod error;
pub mod entity;
pub mod enums;
pub mod dto;
pub mod utils;




pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")
    );
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await.expect("数据库连接失败");
    db
}
