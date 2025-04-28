use crate::api::service::api::{CreateCacheRequest, Notification, NotificationType};
use crate::cache::cache_instance::CacheInstance;
use crate::cluster::Cluster;
use crate::notification::NotificationService;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Clone)]
pub struct CacheService {
    cluster: Cluster,
    caches: Arc<RwLock<HashMap<String, CacheInstance>>>,
    notifications: NotificationService,
}

impl CacheService {
    pub fn new(cluster: Cluster, notifications: NotificationService) -> Self {
        let svc = Self {
            cluster,
            caches: Arc::new(RwLock::new(HashMap::new())),
            notifications: notifications.clone(),
        };
        let register_notifications = notifications.clone();
        let register_svc = svc.clone();
        tokio::spawn(async move {
            let mut subscriber = register_notifications.subscribe();
            let node_found = NotificationType::NodeFound as i32;
            while let Ok(notification) = subscriber.recv().await {
                if notification.notification_type == node_found {
                    register_svc.sync_caches().await;
                }
            }
        });
        svc
    }

    pub async fn create_cache(
        &self,
        id: &str,
        max_capacity: u64,
        ttl: u64,
        tti: u64,
        notify: bool,
    ) -> Result<String, String> {
        let id = id.to_string();
        let mut caches = self.caches.write().await;
        if caches.contains_key(&id) {
            return Ok(id.clone());
        }
        let cache_instance = CacheInstance::new(
            self.cluster.node.clone(),
            self.notifications.clone(),
            id.clone(),
            max_capacity,
            ttl,
            tti,
        );
        let cache_id = cache_instance.id.clone();
        caches.insert(id.clone(), cache_instance);
        if notify {
            let notification = Notification {
                cache: id.to_string(),
                create: Some(CreateCacheRequest {
                    name: id.to_string(),
                    ttl,
                    tti,
                    max_capacity,
                }),
                notification_type: NotificationType::CacheCreated.into(),
                key: None,
                value: None,
                node: Some(self.cluster.node.clone()),
            };
            self.notifications.notify(notification);
        }
        Ok(cache_id)
    }

    pub async fn sync_caches(&self) {
        let caches = self.caches.read().await;
        for (id, cache) in caches.iter() {
            let notification = Notification {
                cache: id.clone(),
                create: Some(CreateCacheRequest {
                    name: id.to_string(),
                    ttl: cache.ttl,
                    tti: cache.tti,
                    max_capacity: cache.max_capacity,
                }),
                notification_type: NotificationType::CacheCreated.into(),
                key: None,
                value: None,
                node: Some(self.cluster.node.clone()),
            };
            self.notifications.notify(notification);
        }
    }

    pub async fn get(&self, cache: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let caches = self.caches.read().await;
        if let Some(cache) = caches.get(cache) {
            Ok(cache.get(key).await)
        } else {
            Err(format!("failed get: cache {} not found", cache))
        }
    }

    pub async fn put(
        &self,
        id: &str,
        key: String,
        value: Vec<u8>,
        notify: bool,
    ) -> Result<(), String> {
        let caches = self.caches.read().await;
        if let Some(cache) = caches.get(id) {
            debug!("put: cache: {id}, key: {key}");
            if self.cluster.is_this_node(&key).await {
                cache.put(key.clone(), value.clone()).await;
            } else {
                warn!("put: cache: {id}, key: {key}, but this node is not the owner of the key, skip put")
            }
            if notify {
                let notification = Notification {
                    cache: id.to_string(),
                    create: None,
                    notification_type: NotificationType::ValueUpdated.into(),
                    key: Some(key),
                    value: Some(value),
                    node: Some(self.cluster.node.clone()),
                };
                self.notifications.notify(notification);
            }
            Ok(())
        } else {
            Err(format!("failed put: cache {} not found", id))
        }
    }

    pub async fn delete(&self, id: &str, key: &str, notify: bool) -> Result<(), String> {
        debug!("delete: cache: {id}, key: {key}");
        let caches = self.caches.read().await;
        if let Some(cache) = caches.get(id) {
            cache.delete(key).await;
            if notify {
                let notification = Notification {
                    cache: id.to_string(),
                    create: None,
                    notification_type: NotificationType::ValueDeleted.into(),
                    key: Some(key.to_string()),
                    value: None,
                    node: Some(self.cluster.node.clone()),
                };
                self.notifications.notify(notification);
            }
            Ok(())
        } else {
            Err(format!("failed delete: cache {} not found", id))
        }
    }

    pub async fn clear(&self, id: &str, notify: bool) -> Result<(), String> {
        info!("clear: cache: {id}");
        let caches = self.caches.read().await;
        if let Some(cache) = caches.get(id) {
            cache.clear().await;
            if notify {
                let notification = Notification {
                    cache: id.to_string(),
                    create: None,
                    notification_type: NotificationType::CacheCleared.into(),
                    key: None,
                    value: None,
                    node: Some(self.cluster.node.clone()),
                };
                self.notifications.notify(notification);
            }
            Ok(())
        } else {
            Err(format!("failed clear: cache {} not found", id))
        }
    }
}
