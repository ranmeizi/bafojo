use crate::config::CFG;
use anyhow::{anyhow, Error, Result};
use chrono::Local;
use once_cell::sync::OnceCell;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;
use std::sync::RwLock;

pub mod user_info;
static GLOBAL_REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::new();
const TIMEOUT: i64 = 1000 * 60 * 5;

fn create_pool() -> Result<Pool<RedisConnectionManager>, Error> {
    let manager = RedisConnectionManager::new(CFG.app.redis_str.clone())?;

    let pool = r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)?;

    Ok(pool)
}

static mut TIME_TO_USE_REDIS: RwLock<i64> = RwLock::new(0);

pub fn get_conn() -> Result<PooledConnection<RedisConnectionManager>> {
    let now = Local::now().timestamp_millis();

    unsafe {
        println!("{},{}", now, *TIME_TO_USE_REDIS.read().unwrap() + TIMEOUT);
        if now < *TIME_TO_USE_REDIS.read().unwrap() + TIMEOUT {
            println!("链接异常");
            return Err(anyhow!("链接异常"));
        }
    }

    match GLOBAL_REDIS_POOL.get_or_try_init(create_pool) {
        Ok(pool) => {
            let conn = pool.get()?;

            Ok(conn)
        }
        Err(e) => {
            unsafe {
                *TIME_TO_USE_REDIS.write().unwrap() = Local::now().timestamp_millis();
            }
            println!("连接池创建失败");
            println!("{}", e);
            Err(anyhow!("连接池创建失败"))
        }
    }
}
