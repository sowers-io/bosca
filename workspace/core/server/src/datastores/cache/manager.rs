use crate::datastores::cache::cache::{BoscaCache, ClearableCache};
use async_graphql::Error;
use async_nats::jetstream::Context;
use async_nats::{jetstream,};
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BoscaCacheManager {
    caches: Arc<Mutex<HashMap<String, Box<dyn ClearableCache + Send + Sync>>>>,
    jetstream: Context,
}

impl BoscaCacheManager {
    pub fn new(jetstream: Context) -> Self {
        Self {
            caches: Arc::new(Mutex::new(HashMap::new())),
            jetstream,
        }
    }

    pub async fn new_cache<V>(
        &mut self,
        name: &str,
        size: u64,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new memory cache: {} with size: {}", name, size);
        let kv = self
            .jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: name.to_string(),
                history: 1,
                max_age: Duration::from_secs(1800),
                ..Default::default()
            })
            .await?;
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            kv,
        );
        caches.insert(name.to_string(), Box::new(cache.clone()) as Box<dyn ClearableCache + Send + Sync>);
        Ok(cache)
    }

    pub async fn new_id_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new memory cache: {} with size: {}", name, size);
        let kv = self
            .jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: name.to_string(),
                ..Default::default()
            })
            .await?;
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            kv,
        );
        caches.insert(name.to_string(), Box::new(cache.clone()));
        Ok(cache)
    }

    pub async fn new_string_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new memory cache: {} with size: {}", name, size);
        let kv = self
            .jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: name.to_string(),
                ..Default::default()
            })
            .await?;
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            kv,
        );
        caches.insert(name.to_string(), Box::new(cache.clone()));
        Ok(cache)
    }

    pub async fn new_int_tiered_cache<V>(
        &mut self,
        name: &str,
        size: u64,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        info!("adding new memory cache: {} with size: {}", name, size);
        let kv = self
            .jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: name.to_string(),
                ..Default::default()
            })
            .await?;
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            kv,
        );
        caches.insert(name.to_string(), Box::new(cache.clone()));
        Ok(cache)
    }

    pub async fn clear_all(&self) {
        let caches = self.caches.lock().await;
        for cache in caches.values() {
            if let Err(e) = cache.clear().await {
                error!("error clearing cache: {:?}", e);
            }
        }
    }

    pub fn watch(&self) {
        let c = Arc::clone(&self.caches);
        tokio::spawn(async move {
            let caches = c.lock().await;
            for cache in caches.values() {
                cache.watch();
            }
        });
    }
}
