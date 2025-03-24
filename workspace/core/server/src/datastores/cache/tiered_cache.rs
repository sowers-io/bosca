use crate::datastores::cache::cache::BoscaCacheInterface;
use crate::datastores::cache::memory_cache::MemoryCache;
use crate::datastores::cache::redis_cache::RedisCache;
use crate::datastores::notifier::Notifier;
use log::error;
use std::hash::Hash;
use std::sync::Arc;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub enum TieredCacheType {
    Metadata,
    MetadataSupplementary,
    Collection,
    CollectionSupplementary
}

#[derive(Clone)]
pub struct TieredCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    memory: MemoryCache<K, V>,
    redis: RedisCache,
}

impl<K, V> TieredCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    pub fn new(memory: MemoryCache<K, V>, redis: RedisCache) -> Self {
        Self {
            memory: memory.clone(),
            redis,
        }
    }
}

impl<V> TieredCache<Uuid, V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    pub fn watch_changes(&self, notifier: Arc<Notifier>, tiered_cache: TieredCacheType) {
        let memory = self.memory.clone();
        tokio::spawn(async move {
            loop {
                match tiered_cache {
                    TieredCacheType::Metadata => {
                        if let Ok(stream) = notifier.listen_metadata_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                if let Ok(id) = Uuid::parse_str(&item) {
                                    memory.remove(&id).await;
                                }
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    TieredCacheType::Collection => {
                        if let Ok(stream) = notifier.listen_collection_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                if let Ok(id) = Uuid::parse_str(&item) {
                                    memory.remove(&id).await;
                                }
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

impl<V> TieredCache<String, V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    pub fn watch_changes(&self, notifier: Arc<Notifier>, tiered_cache: TieredCacheType) {
        let memory = self.memory.clone();
        tokio::spawn(async move {
            loop {
                match tiered_cache {
                    TieredCacheType::MetadataSupplementary => {
                        if let Ok(stream) = notifier.listen_metadata_supplementary_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                let key = format!("{}:{}", item.id, item.key);
                                memory.remove(&key).await;
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    TieredCacheType::CollectionSupplementary => {
                        if let Ok(stream) = notifier.listen_collection_supplementary_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                let key = format!("{}:{}", item.id, item.key);
                                memory.remove(&key).await;
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

#[async_trait::async_trait]
impl<K, V> BoscaCacheInterface<K, V> for TieredCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    async fn get(&self, key: &K) -> Option<V> {
        if let Some(value) = self.memory.get(key).await {
            return Some(value);
        }
        if let Some(value) = self.redis.get(key).await {
            return Some(value);
        }
        None
    }

    async fn set(&self, key: &K, value: &V) {
        self.memory.set(key, value).await;
        self.redis.set(key, value).await;
    }

    async fn remove(&self, key: &K) {
        self.memory.remove(key).await;
        <RedisCache as BoscaCacheInterface<K, V>>::remove::<'_, '_, '_>(&self.redis, key).await;
    }

    async fn clear(&self) {
        self.memory.clear().await;
        <RedisCache as BoscaCacheInterface<K, V>>::clear::<'_, '_>(&self.redis).await;
    }
}
