use super::StructCache;
use mobc::Pool;
use mobc_redis::redis::{self};
use mobc_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};
use tokio::sync::OnceCell;

const TEST_KEY: &'static str = "mobc::redis::test";

pub static R_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn redis_conn() -> Pool<RedisConnectionManager> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool: Pool<RedisConnectionManager> = Pool::builder().max_open(20).build(manager);
    pool
}

// struct RedisStructCache {}

// impl<T> StructCache<T> for RedisStructCache
// where
//     T: Serialize + 'static,
// {
//     type Future = Pin<Box<dyn Future<Output = Result<T, ()>> + Send>>;

//     fn get(self, key: &str) -> Self::Future {
//         Box::pin(async move {})
//     }

//     fn set(self, key: &str, data: T) -> () {}

//     fn del(self, key: &str) -> () {}
// }

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_set() {
        let pool = R_POOL.get_or_init(redis_conn).await;

        let mut conn = pool.get().await.unwrap();

        conn.set()
    }
}
