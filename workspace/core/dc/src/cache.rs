//! Cache service module using moka
//! 
//! This module provides the in-memory cache functionality using moka.
//! It supports operations like create cache, get, put, delete, and clear.

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};
use moka::future::Cache;
use uuid::Uuid;
use crate::cluster::cluster::Cluster;
use crate::notification::{Notification, NotificationType};

/// Represents a single cache instance
pub struct CacheInstance {
    id: String,
    name: String,
    cache: Cache<String, Vec<u8>>,
}

impl CacheInstance {
    /// Creates a new cache instance
    pub fn new(name: String, max_capacity: u64) -> Self {
        let id = Uuid::new_v4().to_string();
        let cache = Cache::new(max_capacity);

        Self {
            id,
            name,
            cache,
        }
    }

    /// Gets a value from the cache
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.get(key).await
    }

    /// Puts a value in the cache
    pub async fn put(&self, key: String, value: Vec<u8>) {
        self.cache.insert(key, value).await;
    }

    /// Deletes a value from the cache
    pub async fn delete(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    /// Clears the entire cache
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}

/// Service that manages multiple cache instances
pub struct CacheService {
    cluster: Cluster,
    caches: RwLock<HashMap<String, CacheInstance>>,
    pub notification_tx: Arc<broadcast::Sender<Notification>>,
}

impl CacheService {
    /// Creates a new cache service
    pub fn new(cluster: Cluster, notification_tx: Arc<broadcast::Sender<Notification>>) -> Arc<Self> {
        Arc::new(Self {
            cluster,
            caches: RwLock::new(HashMap::new()),
            notification_tx,
        })
    }

    /// Creates a new cache instance
    pub async fn create_cache(&self, name: String, max_capacity: u64) -> Result<String, String> {
        let cache_instance = CacheInstance::new(name.clone(), max_capacity);
        let cache_id = cache_instance.id.clone();

        // Propose the cache creation to the cluster
        let data = format!("create_cache:{}:{}", name, max_capacity).into_bytes();
        // self.cluster.propose(data).await?;

        // Add the cache to our local state
        let mut caches = self.caches.write().await;
        caches.insert(cache_id.clone(), cache_instance);

        // Send notification
        let notification = Notification {
            cache_id: cache_id.clone(),
            notification_type: NotificationType::CacheCreated,
            key: None,
        };
        let _ = self.notification_tx.send(notification);

        Ok(cache_id)
    }

    /// Gets a value from a cache
    pub async fn get(&self, cache_id: &str, key: &str) -> Option<Vec<u8>> {
        let caches = self.caches.read().await;

        if let Some(cache) = caches.get(cache_id) {
            cache.get(key).await
        } else {
            None
        }
    }

    /// Puts a value in a cache
    pub async fn put(&self, cache_id: &str, key: String, value: Vec<u8>) -> Result<(), String> {
        // Propose the put operation to the cluster
        let data = format!("put:{}:{}:{}", cache_id, key, base64::encode(&value)).into_bytes();
        // self.cluster.propose(data).await?;

        // Update local cache
        let caches = self.caches.read().await;

        if let Some(cache) = caches.get(cache_id) {
            cache.put(key.clone(), value).await;

            // Send notification
            let notification = Notification {
                cache_id: cache_id.to_string(),
                notification_type: NotificationType::ValueUpdated,
                key: Some(key),
            };
            let _ = self.notification_tx.send(notification);

            Ok(())
        } else {
            Err(format!("Cache with ID {} not found", cache_id))
        }
    }

    /// Deletes a value from a cache
    pub async fn delete(&self, cache_id: &str, key: &str) -> Result<(), String> {
        // Propose the delete operation to the cluster
        let data = format!("delete:{}:{}", cache_id, key).into_bytes();
        // self.cluster.propose(data).await?;

        // Update local cache
        let caches = self.caches.read().await;

        if let Some(cache) = caches.get(cache_id) {
            cache.delete(key).await;

            // Send notification
            let notification = Notification {
                cache_id: cache_id.to_string(),
                notification_type: NotificationType::ValueDeleted,
                key: Some(key.to_string()),
            };
            let _ = self.notification_tx.send(notification);

            Ok(())
        } else {
            Err(format!("Cache with ID {} not found", cache_id))
        }
    }

    /// Clears a cache
    pub async fn clear(&self, cache_id: &str) -> Result<(), String> {
        // Propose the clear operation to the cluster
        let data = format!("clear:{}", cache_id).into_bytes();
        // self.cluster.propose(data).await?;

        // Update local cache
        let caches = self.caches.read().await;

        if let Some(cache) = caches.get(cache_id) {
            cache.clear().await;

            // Send notification
            let notification = Notification {
                cache_id: cache_id.to_string(),
                notification_type: NotificationType::CacheCleared,
                key: None,
            };
            let _ = self.notification_tx.send(notification);

            Ok(())
        } else {
            Err(format!("Cache with ID {} not found", cache_id))
        }
    }
}
