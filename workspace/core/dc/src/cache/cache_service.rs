use crate::api::service::api::{CreateCacheRequest, Notification, NotificationType};
use crate::cache::cache_configuration::CacheConfiguration;
use crate::cache::cache_instance::CacheInstance;
use crate::cache::store::Store;
use crate::cluster::Cluster;
use crate::notification::NotificationService;
use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Clone)]
pub struct CacheService {
    cluster: Cluster,
    caches: Arc<RwLock<HashMap<String, CacheInstance>>>,
    notifications: NotificationService,
    store: Store,
}

impl CacheService {
    pub async fn new(
        cluster: Cluster,
        notifications: NotificationService,
        store: Store,
    ) -> Result<Self, Box<dyn Error>> {
        let svc = Self {
            cluster,
            caches: Arc::new(RwLock::new(HashMap::new())),
            notifications: notifications.clone(),
            store,
        };

        let configurations = svc.store.load().await?;
        for cfg in configurations {
            svc.create_cache(cfg, false).await?;
        }

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
        Ok(svc)
    }

    pub async fn create_cache(
        &self,
        configuration: CacheConfiguration,
        notify: bool,
    ) -> Result<String, Box<dyn Error>> {
        let mut caches = self.caches.write().await;
        if caches.contains_key(&configuration.id) {
            return Ok(configuration.id.clone());
        }
        let cache_instance = CacheInstance::new(
            self.cluster.node.clone(),
            self.notifications.clone(),
            configuration.clone(),
        );
        caches.insert(configuration.id.clone(), cache_instance);
        self.store.save(configuration.clone()).await?;
        if notify {
            let notification = Notification {
                cache: configuration.id.clone(),
                create: Some(CreateCacheRequest {
                    name: configuration.id.clone(),
                    ttl: configuration.ttl,
                    tti: configuration.tti,
                    max_capacity: configuration.max_capacity,
                }),
                notification_type: NotificationType::CacheCreated.into(),
                key: None,
                value: None,
                node: Some(self.cluster.node.clone()),
            };
            self.notifications.notify(notification);
        }
        Ok(configuration.id.clone())
    }

    pub async fn sync_caches(&self) {
        let caches = self.caches.read().await;
        for (id, cache) in caches.iter() {
            let notification = Notification {
                cache: id.clone(),
                create: Some(CreateCacheRequest {
                    name: id.to_string(),
                    ttl: cache.configuration.ttl,
                    tti: cache.configuration.tti,
                    max_capacity: cache.configuration.max_capacity,
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
            warn!("get: cache: {cache} not found");
            Ok(None)
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
            self.put_internal(id, key, value, notify, cache).await;
        } else {
            warn!("get: cache: {id} not found");
        }
        Ok(())
    }

    async fn put_internal(
        &self,
        id: &str,
        key: String,
        value: Vec<u8>,
        notify: bool,
        cache: &CacheInstance,
    ) {
        debug!("put: cache: {id}, key: {key}");
        if self.cluster.is_this_node(&key).await {
            cache.put(key.clone(), value.clone()).await;
        } else {
            warn!(
                "put: cache: {id}, key: {key}, but this node is not the owner of the key, skip put"
            )
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
        } else {
            warn!("failed delete: cache {} not found", id)
        }
        Ok(())
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
        } else {
            warn!("failed clear: cache {} not found", id)
        }
        Ok(())
    }
}
