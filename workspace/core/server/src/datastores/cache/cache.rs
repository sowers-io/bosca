use crate::datastores::cache::memory_cache::MemoryCache;
use crate::datastores::cache::redis_cache::RedisCache;
use std::hash::Hash;

#[async_trait::async_trait]
pub trait ManagedBoscaCache: Send + Sync {
    fn to_managed(&self) -> Box<dyn ManagedBoscaCache>;

    fn watch(&self);

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

    fn watch(&self);
}

#[derive(Clone, Debug)]
pub enum BoscaCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    MemoryCache(MemoryCache<K, V>),
    // TieredCache(TieredCache<K, V>),
    RedisCache(RedisCache),
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

    fn watch(&self) {
        match self {
            BoscaCache::MemoryCache(c) => c.watch(),
            // BoscaCache::TieredCache(c) => c.watch(),
            BoscaCache::RedisCache(c) => <RedisCache as BoscaCacheInterface<K, V>>::watch(c),
        }
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
            // BoscaCache::TieredCache(cache) => cache.get(key).await,
            BoscaCache::RedisCache(cache) => cache.get(&key).await,
        }
    }

    async fn set(&self, key: &K, value: &V) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.set(key, value).await,
            // BoscaCache::TieredCache(cache) => cache.set(key, value).await,
            BoscaCache::RedisCache(cache) => cache.set(key, value).await,
        }
    }

    async fn remove(&self, key: &K) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.remove(key).await,
            // BoscaCache::TieredCache(cache) => cache.remove(key).await,
            BoscaCache::RedisCache(cache) => {
                <RedisCache as BoscaCacheInterface<K, V>>::remove::<'_, '_, '_>(cache, key).await
            }
        }
    }

    async fn clear(&self) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.clear().await,
            // BoscaCache::TieredCache(cache) => cache.clear().await,
            BoscaCache::RedisCache(cache) => {
                <RedisCache as BoscaCacheInterface<K, V>>::clear::<'_, '_>(cache).await
            }
        }
    }

    fn watch(&self) {
        match self {
            BoscaCache::MemoryCache(cache) => cache.watch(),
            // BoscaCache::TieredCache(cache) => cache.watch(),
            BoscaCache::RedisCache(cache) => {
                <RedisCache as BoscaCacheInterface<K, V>>::watch(cache)
            }
        }
    }
}
