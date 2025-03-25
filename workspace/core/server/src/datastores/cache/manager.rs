use crate::datastores::cache::cache::{BoscaCache, ManagedBoscaCache};
use crate::datastores::cache::memory_cache::MemoryCache;
use crate::datastores::cache::redis_cache::RedisCache;
use crate::datastores::cache::tiered_cache::{TieredCache, TieredCacheType};
use crate::datastores::notifier::Notifier;
use crate::redis::RedisClient;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use log::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct BoscaCacheManager {
    redis: RedisClient,
    notifier: Arc<Notifier>,
    caches: HashMap<String, Box<dyn ManagedBoscaCache>>,
}

impl BoscaCacheManager {
    pub fn new(redis: RedisClient, notifier: Arc<Notifier>) -> Self {
        Self {
            redis,
            notifier,
            caches: HashMap::new(),
        }
    }

    pub fn new_cache<K, V>(&mut self, name: &str, size: u64) -> BoscaCache<K, V>
    where
        K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new memory cache: {} with size: {}", name, size);
        let cache = BoscaCache::MemoryCache(MemoryCache::<K, V>::new(size));
        info!("storing cache added");
        self.caches.insert(name.to_string(), cache.to_managed());
        info!("cache added");
        cache
    }

    pub fn new_id_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        tiered_cache: TieredCacheType,
    ) -> BoscaCache<Uuid, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new id tiered cache: {} with size: {}", name, size);
        let memory_cache = MemoryCache::<Uuid, V>::new_ttl(size, Duration::from_secs(3600));
        let redis_cache = RedisCache::new(self.redis.clone(), name.to_string());
        let cache = TieredCache::<Uuid, V>::new(memory_cache, redis_cache);
        cache.watch_changes(Arc::clone(&self.notifier), tiered_cache);
        let tiered_cache = BoscaCache::TieredCache(cache);
        info!("storing cache added");
        self.caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub fn new_string_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        tiered_cache: TieredCacheType,
    ) -> BoscaCache<String, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new string tiered cache: {} with size: {}", name, size);
        let memory_cache = MemoryCache::<String, V>::new_ttl(size, Duration::from_secs(3600));
        let redis_cache = RedisCache::new(self.redis.clone(), name.to_string());
        let cache = TieredCache::<String, V>::new(memory_cache, redis_cache);
        cache.watch_changes(Arc::clone(&self.notifier), tiered_cache);
        let tiered_cache = BoscaCache::TieredCache(cache);
        info!("storing cache added");
        self.caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub fn new_int_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
        tiered_cache: TieredCacheType,
    ) -> BoscaCache<i64, V>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new int tiered cache: {} with size: {}", name, size);
        let memory_cache = MemoryCache::<i64, V>::new_ttl(size, Duration::from_secs(3600));
        let redis_cache = RedisCache::new(self.redis.clone(), name.to_string());
        let cache = TieredCache::<i64, V>::new(memory_cache, redis_cache);
        cache.watch_changes(Arc::clone(&self.notifier), tiered_cache);
        let tiered_cache = BoscaCache::TieredCache(cache);
        info!("storing cache added");
        self.caches.insert(name.to_string(), tiered_cache.to_managed());
        info!("cache added");
        tiered_cache
    }

    pub async fn clear_all(&self) {
        for (_, cache) in self.caches.iter() {
            cache.clear().await;
        }
    }
}
