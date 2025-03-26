use crate::datastores::cache::cache::BoscaCacheInterface;
use moka::future::Cache;
use std::hash::Hash;
use std::time::Duration;

#[derive(Clone)]
pub struct MemoryCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    cache: Cache<K, V>,
}

impl<K, V> MemoryCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    pub fn new(size: u64) -> Self {
        Self {
            cache: Cache::builder().max_capacity(size).build(),
        }
    }

    pub fn new_ttl(size: u64, ttl: Duration) -> Self {
        Self {
            cache: Cache::builder()
                .max_capacity(size)
                .time_to_live(ttl)
                .build(),
        }
    }
}

#[async_trait::async_trait]
impl<K, V> BoscaCacheInterface<K, V> for MemoryCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    async fn set(&self, key: &K, value: &V) {
        self.cache.insert(key.clone(), value.clone()).await;
    }

    async fn remove(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    async fn clear(&self) {
        self.cache.invalidate_all();
    }

    fn watch(&self) {
    }
}
