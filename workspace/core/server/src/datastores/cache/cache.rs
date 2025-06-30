use crate::redis::RedisClient;
use async_graphql::Error;
use log::error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use bytes::Bytes;
use redis::{AsyncCommands, ToRedisArgs};

#[derive(Clone)]
pub struct BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    name: String,
    redis: RedisClient,
    phantom_data: PhantomData<V>,
}

#[async_trait::async_trait]
pub trait ClearableCache {
    async fn clear(&self) -> Result<(), Error>;
}

#[async_trait::async_trait]
impl<V> ClearableCache for BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    async fn clear(&self) -> Result<(), Error> {
        let mut conn = self.redis.get_manager().await?;
        let keys: Vec<String> = conn.hkeys("bosca:cache:*").await?;
        for key in keys {
            conn.del::<String, i32>(key).await?;
        }
        Ok(())
    }
}

impl<V> Debug for BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryCache").finish()
    }
}

impl<V> BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    pub async fn new_ttl(name: String, redis: RedisClient) -> Result<Self, Error> {
        Ok(Self { name, redis, phantom_data: PhantomData })
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn get<K>(&self, key: &K) -> Option<V>
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone + ToRedisArgs,
    {
        let hkey = format!("bosca:cache:{}", &self.name);
        if let Ok(mut conn) = self.redis.get_manager().await {
            if let Ok(result) = redis::cmd("HGET").arg(hkey).arg(key).query_async(&mut conn).await {
                let bytes: Bytes = result;
                if let Ok(result) = serde_json::from_slice(bytes.as_ref()) {
                    if let Ok(value) = serde_json::from_value(result) {
                        return Some(value);
                    }
                }
            }
        }
        None
    }

    #[tracing::instrument(skip(self, key, value))]
    pub async fn set<K>(&self, key: &K, value: &V)
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone + ToRedisArgs,
    {
        let hkey = format!("bosca:cache:{}", &self.name);
        if let Ok(mut conn) = self.redis.get_manager().await {
            let value = serde_json::to_vec(value).expect("failed to serialize value");
            if let Err(e) = conn.hset::<String, &K, Vec<u8>, i32>(hkey, key, value).await {
                error!("error setting in cache: {e:?}");
            }
        }
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn remove<K>(&self, key: &K)
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone + ToRedisArgs,
    {
        let hkey = format!("bosca:cache:{}", &self.name);
        if let Ok(mut conn) = self.redis.get_manager().await {
            if let Err(e) = conn.hdel::<String, &K, i32>(hkey, key).await {
                error!("error removing from cache: {e:?}");
            }
        }
    }
}
