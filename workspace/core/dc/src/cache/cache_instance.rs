use crate::api::service::api::{Node, Notification, NotificationType};
use crate::cache::cache_configuration::CacheConfiguration;
use crate::notification::NotificationService;
use moka::future::{Cache, CacheBuilder};
use moka::policy::EvictionPolicy;
use std::sync::Arc;
use tracing::info;

pub struct CacheInstance {
    cache: Cache<String, Vec<u8>>,
    pub configuration: CacheConfiguration,
}

impl CacheInstance {
    pub fn new(
        node: Node,
        notifications: NotificationService,
        configuration: CacheConfiguration,
    ) -> Self {
        let cfg = configuration.clone();
        let mut builder = CacheBuilder::new(configuration.max_capacity)
            .eviction_listener(move |k: Arc<String>, v, cause| {
                if cause.was_evicted() {
                    info!(
                        "Cache {} evicted key {} with cause {:?}",
                        cfg.id, k, cause
                    );
                    let notification = Notification {
                        cache: cfg.id.clone(),
                        create: None,
                        notification_type: NotificationType::ValueDeleted as i32,
                        key: Some(k.to_string()),
                        value: Some(v),
                        node: Some(node.clone()),
                    };
                    notifications.notify(notification);
                }
            })
            .eviction_policy(EvictionPolicy::tiny_lfu());
        if configuration.ttl > 0 {
            builder = builder.time_to_live(std::time::Duration::from_secs(configuration.ttl));
        }
        if configuration.tti > 0 {
            builder = builder.time_to_live(std::time::Duration::from_secs(configuration.tti));
        }
        Self {
            cache: builder.build(),
            configuration,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.get(key).await
    }

    pub async fn put(&self, key: String, value: Vec<u8>) {
        self.cache.insert(key.clone(), value.clone()).await;
    }

    pub async fn delete(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}
