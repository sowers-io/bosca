use crate::datastores::cache::memory_cache::MemoryCache;
use std::hash::Hash;

#[async_trait::async_trait]
pub trait ManagedBoscaCache: Send + Sync {
    fn to_managed(&self) -> Box<dyn ManagedBoscaCache>;

    async fn clear(&self);
}

impl Clone for Box<dyn ManagedBoscaCache> {
    fn clone(&self) -> Self {
        self.to_managed()
    }
}

#[async_trait::async_trait]
pub trait BoscaCacheInterface<K, V>: Send + Sync + Clone + Sized
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    async fn get(&self, key: &K) -> Option<V>;

    async fn set(&self, key: &K, value: &V);

    async fn remove(&self, key: &K);

    async fn clear(&self);
}

#[derive(Clone, Debug)]
pub enum BoscaCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    MemoryCache(MemoryCache<K, V>),
}

#[async_trait::async_trait]
impl<K, V> ManagedBoscaCache for BoscaCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    fn to_managed(&self) -> Box<dyn ManagedBoscaCache> {
        Box::new(self.clone())
    }

    async fn clear(&self) {
        BoscaCacheInterface::clear(self).await;
    }
}

#[async_trait::async_trait]
impl<K, V> BoscaCacheInterface<K, V> for BoscaCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    async fn get(&self, key: &K) -> Option<V> {
        match self {
            BoscaCache::MemoryCache(cache) => cache.get(key).await,
        }
    }

    async fn set(&self, key: &K, value: &V) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.set(key, value).await,
        }
    }

    async fn remove(&self, key: &K) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.remove(key).await,
        }
    }

    async fn clear(&self) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.clear().await,
        }
    }
}
