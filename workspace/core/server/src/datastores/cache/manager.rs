use crate::datastores::cache::cache::{BoscaCache, ClearableCache};
use async_graphql::Error;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BoscaCacheManager {
    caches: Arc<Mutex<HashMap<String, Box<dyn ClearableCache + Send + Sync>>>>,
    client: bosca_dc_client::client::Client,
}

impl BoscaCacheManager {
    pub fn new(client: bosca_dc_client::client::Client) -> Self {
        Self {
            caches: Arc::new(Mutex::new(HashMap::new())),
            client,
        }
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
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            self.client.clone(),
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
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            self.client.clone(),
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
        let mut caches = self.caches.lock().await;
        let cache = BoscaCache::<V>::new_ttl(
            name.to_string(),
            size,
            Duration::from_secs(1800),
            self.client.clone(),
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
