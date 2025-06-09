use crate::datastores::cache::cache::{BoscaCache, ClearableCache};
use async_graphql::Error;
use log::error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::redis::RedisClient;

#[derive(Clone)]
pub struct BoscaCacheManager {
    caches: Arc<Mutex<HashMap<String, Box<dyn ClearableCache + Send + Sync>>>>,
    client: RedisClient,
}

impl BoscaCacheManager {
    pub fn new(client: RedisClient) -> Self {
        Self {
            caches: Arc::new(Mutex::new(HashMap::new())),
            client,
        }
    }

    pub async fn get_cache_names(&self) -> Result<Vec<String>, Error> {
        let caches = self.caches.lock().await;
        Ok(caches.keys().map(|k| k.to_string()).collect())
    }

    pub async fn new_id_tiered_cache<V>(
        &mut self,
        name: &str,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            self.client.clone(),
        ).await?;
        caches.insert(name.to_string(), Box::new(cache.clone()));
        Ok(cache)
    }

    pub async fn new_string_tiered_cache<V>(
        &mut self,
        name: &str,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            self.client.clone(),
        ).await?;
        caches.insert(name.to_string(), Box::new(cache.clone()));
        Ok(cache)
    }

    pub async fn new_int_tiered_cache<V>(
        &mut self,
        name: &str,
    ) -> Result<BoscaCache<V>, Error>
    where
        V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
    {
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            self.client.clone(),
        ).await?;
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
}
