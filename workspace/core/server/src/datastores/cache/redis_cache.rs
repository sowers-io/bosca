use crate::datastores::cache::cache::BoscaCacheInterface;
use crate::redis::RedisClient;
use log::error;
use redis::AsyncCommands;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

#[derive(Clone)]
pub struct RedisCache {
    name: String,
    redis: RedisClient,
}

impl Debug for RedisCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisCache").finish()
    }
}

impl RedisCache {
    pub fn new(redis: RedisClient, name: String) -> Self {
        Self {
            redis,
            name: format!("cache::{}", name),
        }
    }
}

#[async_trait::async_trait]
impl<K, V> BoscaCacheInterface<K, V> for RedisCache
where
    K: Clone + Send + Sync + serde::ser::Serialize + Hash + Eq + redis::ToRedisArgs,
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    #[tracing::instrument(skip(self, key))]
    async fn get(&self, key: &K) -> Option<V> {
        let Ok(redis) = self.redis.get().await else {
            return None;
        };
        let Ok(mut connection) = redis.get_connection().await else {
            return None;
        };
        let Ok(value) = connection.get::<K, String>(key.clone()).await else {
            return None;
        };
        let Ok(value) = serde_json::from_str(&value) else {
            error!("failed to convert value: {:?}", value);
            return None;
        };
        Some(value)
    }

    #[tracing::instrument(skip(self, key, value))]
    async fn set(&self, key: &K, value: &V) {
        let Ok(redis) = self.redis.get().await else {
            return;
        };
        let Ok(mut connection) = redis.get_connection().await else {
            return;
        };
        let Ok(str) = serde_json::to_string(value) else {
            return;
        };
        if let Err(e) = connection
            .hset::<String, K, String, i32>(self.name.clone(), key.clone(), str)
            .await
        {
            error!("failed to set key: {:?}", e);
        }
    }

    #[tracing::instrument(skip(self, key))]
    async fn remove(&self, key: &K) {
        let Ok(redis) = self.redis.get().await else {
            return;
        };
        let Ok(mut connection) = redis.get_connection().await else {
            return;
        };
        if let Err(e) = connection
            .hdel::<String, K, i32>(self.name.clone(), key.clone())
            .await
        {
            error!("failed to set key: {:?}", e);
        }
    }

    #[tracing::instrument(skip(self))]
    async fn clear(&self) {
        let Ok(redis) = self.redis.get().await else {
            return;
        };
        let Ok(mut connection) = redis.get_connection().await else {
            return;
        };
        if let Err(e) = connection.del::<String, i32>(self.name.clone()).await {
            error!("failed to clear: {:?}", e);
        }
    }

    fn watch(&self) {}
}
