use crate::api::service::api::{Node, Notification, NotificationType};
use crate::notification::NotificationService;
use moka::future::{Cache, CacheBuilder};
use moka::policy::EvictionPolicy;

pub struct CacheInstance {
    pub(crate) id: String,
    cache: Cache<String, Vec<u8>>,
    pub max_capacity: u64,
    pub ttl: u64,
    pub tti: u64,
}

impl CacheInstance {
    pub fn new(
        node: Node,
        notifications: NotificationService,
        id: String,
        max_capacity: u64,
        ttl: u64,
        tti: u64,
    ) -> Self {
        let cache_id = id.to_string();
        let mut builder = CacheBuilder::new(max_capacity)
            .eviction_listener(move |k, v, cause| {
                let notification = Notification {
                    cache: cache_id,
                    max_capacity,
                    notification_type: NotificationType::ValueDeleted as i32,
                    key: Some(k.to_string()),
                    value: Some(v),
                    node: Some(node.clone()),
                };
                notifications.notify(notification);
            })
            .eviction_policy(EvictionPolicy::tiny_lfu());
        if ttl > 0 {
            builder = builder.time_to_live(std::time::Duration::from_secs(ttl));
        }
        if tti > 0 {
            builder = builder.time_to_live(std::time::Duration::from_secs(tti));
        }
        Self {
            id,
            max_capacity,
            ttl,
            tti,
            cache: builder.build(),
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
