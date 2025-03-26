use crate::datastores::cache::cache::BoscaCacheInterface;
use crate::datastores::cache::memory_cache::MemoryCache;
use crate::datastores::cache::redis_cache::RedisCache;
use crate::datastores::notifier::Notifier;
use log::error;
use std::hash::Hash;
use std::sync::Arc;
use tokio_stream::StreamExt;
use uuid::Uuid;

#[derive(Clone)]
pub enum TieredCacheType {
    Slug,
    Metadata,
    Collection,
    State,
    Transition,
    StorageSystem,
    Model,
    Prompt,
    Workflow,
    Trait,
    Activity,
    WorkflowActivity,
}

#[derive(Clone)]
pub struct TieredCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs + 'static,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    memory: MemoryCache<K, V>,
    redis: RedisCache,
    tiered_type: TieredCacheType,
    notifier: Arc<Notifier>,
}

impl<K, V> TieredCache<K, V>
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    pub fn new(
        memory: MemoryCache<K, V>,
        redis: RedisCache,
        tiered_type: TieredCacheType,
        notifier: Arc<Notifier>,
    ) -> Self {
        Self {
            memory,
            redis,
            tiered_type,
            notifier,
        }
    }
}

impl<V> TieredCache<Uuid, V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    fn watch_changes(&self) {
        let memory = self.memory.clone();
        let tiered_cache = self.tiered_type.clone();
        let notifier = Arc::clone(&self.notifier);
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
                    TieredCacheType::State => {
                        if let Ok(stream) = notifier.listen_state_changes().await {
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
    fn watch_changes(&self) {
        let memory = self.memory.clone();
        let tiered_cache = self.tiered_type.clone();
        let notifier = Arc::clone(&self.notifier);
        tokio::spawn(async move {
            loop {
                match tiered_cache {
                    TieredCacheType::Workflow => {
                        if let Ok(stream) = notifier.listen_workflow_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                memory.remove(&item).await;
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    TieredCacheType::Transition => {
                        if let Ok(stream) = notifier.listen_transition_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                let key = format!("{}-{}", item.from_state_id, item.to_state_id);
                                memory.remove(&key).await;
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    TieredCacheType::Trait => {
                        if let Ok(stream) = notifier.listen_trait_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                memory.remove(&item).await;
                            }
                        } else {
                            error!("failed to listen for metadata changes, trying again")
                        }
                    }
                    TieredCacheType::Activity => {
                        if let Ok(stream) = notifier.listen_activity_changes().await {
                            tokio::pin!(stream);
                            while let Some(item) = stream.next().await {
                                memory.remove(&item).await;
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

impl<V> TieredCache<i64, V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    fn watch_changes(&self) {
        let memory = self.memory.clone();
        let tiered_cache = self.tiered_type.clone();
        let notifier = Arc::clone(&self.notifier);
        tokio::spawn(async move {
            loop {
                if let TieredCacheType::WorkflowActivity = tiered_cache {
                    if let Ok(stream) = notifier.listen_workflow_activity_changes().await {
                        tokio::pin!(stream);
                        while let Some(item) = stream.next().await {
                            memory.remove(&item).await;
                        }
                    } else {
                        error!("failed to listen for metadata changes, trying again")
                    }
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

    fn watch(&self) {
        let type_name = std::any::type_name::<K>();
        if type_name.ends_with("::Uuid") {
            #[allow(clippy::transmute_ptr_to_ptr)]
            let uuid_cache =
                unsafe { std::mem::transmute::<&TieredCache<K, V>, &TieredCache<Uuid, V>>(self) };
            uuid_cache.watch_changes();
        } else if type_name.ends_with("::String") {
            #[allow(clippy::transmute_ptr_to_ptr)]
            let string_cache =
                unsafe { std::mem::transmute::<&TieredCache<K, V>, &TieredCache<String, V>>(self) };
            string_cache.watch_changes();
        } else if type_name == "i64" {
            #[allow(clippy::transmute_ptr_to_ptr)]
            let i64_cache =
                unsafe { std::mem::transmute::<&TieredCache<K, V>, &TieredCache<i64, V>>(self) };
            i64_cache.watch_changes();
        }
    }
}
