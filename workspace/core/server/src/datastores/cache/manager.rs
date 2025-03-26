use crate::datastores::cache::cache::{BoscaCache, ManagedBoscaCache};
use crate::datastores::cache::memory_cache::MemoryCache;
use crate::datastores::cache::redis_cache::RedisCache;
use crate::datastores::cache::tiered_cache::TieredCacheType;
use crate::datastores::notifier::Notifier;
use crate::redis::RedisClient;
use log::info;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct BoscaCacheManager {
    redis: RedisClient,
    caches: Arc<Mutex<HashMap<String, Box<dyn ManagedBoscaCache>>>>,
}

impl BoscaCacheManager {
    pub fn new(redis: RedisClient, _: Arc<Notifier>) -> Self {
        Self {
            redis,
            caches: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn new_cache<K, V>(&mut self, name: &str, size: u64) -> BoscaCache<K, V>
    where
        K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        info!("adding new memory cache: {} with size: {}", name, size);
        let cache = BoscaCache::MemoryCache(MemoryCache::<K, V>::new_ttl(size, Duration::from_secs(1800)));
        info!("storing cache added");
        caches.insert(name.to_string(), cache.to_managed());
        info!("cache added");
        cache
    }

    pub async fn new_id_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        _: TieredCacheType,
    ) -> BoscaCache<Uuid, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        info!(
            "adding new string tiered cache: {} with size: {}",
            name, size
        );
        let cache = RedisCache::new(self.redis.clone(), name.to_string());
        let tiered_cache = BoscaCache::RedisCache(cache);
        info!("storing cache added");
        caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub async fn new_string_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        _: TieredCacheType,
    ) -> BoscaCache<String, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        info!(
            "adding new string tiered cache: {} with size: {}",
            name, size
        );
        let cache = RedisCache::new(self.redis.clone(), name.to_string());
        let tiered_cache = BoscaCache::RedisCache(cache);
        info!("storing cache added");
        caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub async fn new_int_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        _: TieredCacheType,
    ) -> BoscaCache<i64, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        info!(
            "adding new string tiered cache: {} with size: {}",
            name, size
        );
        let cache = RedisCache::new(self.redis.clone(), name.to_string());
        let tiered_cache = BoscaCache::RedisCache(cache);
        info!("storing cache added");
        caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub async fn clear_all(&self) {
        let caches = self.caches.lock().await;
        for cache in caches.values() {
            cache.clear().await;
        }
    }

    pub async fn watch_all(&self) {
        info!("watching all caches");
        let caches = self.caches.lock().await;
        for cache in caches.values() {
            cache.watch();
        }
        info!("all caches are being watched");
    }
}
