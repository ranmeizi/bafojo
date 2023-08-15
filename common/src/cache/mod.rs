use serde::Serialize;
use std::future::Future;

mod redis;

pub trait StructCache<T: Serialize + 'static> {
    type Future: Future<Output = Result<T, ()>> + Send + 'static;
    /**
     * 使用 key 获取 缓存的值
     */
    fn get(self, key: &str) -> Self::Future;

    /**
     * 更新缓存
     */
    fn set(self, key: &str, data: T) -> ();

    /**
     * 使用 key 删除缓存
     */
    fn del(self, key: &str) -> ();
}
