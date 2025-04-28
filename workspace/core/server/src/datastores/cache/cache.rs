use async_graphql::Error;
use bosca_dc_client::client::Client;
use log::error;
use moka::future::Cache;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    name: String,
    cache: Arc<Cache<String, V>>,
    client: Client,
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
        self.cache.invalidate_all();
        self.client.clear(&self.name).await?;
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
    pub fn new_ttl(name: String, size: u64, ttl: Duration, client: Client) -> Self {
        Self {
            name,
            cache: Arc::new(
                Cache::builder()
                    .max_capacity(size)
                    .time_to_idle(ttl)
                    .build(),
            ),
            client,
        }
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn get<K>(&self, key: &K) -> Option<V>
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        let value = self.cache.get(&key).await;
        if value.is_some() {
            value
        } else if let Ok(Some(data)) = self.client.get(&self.name, &key).await {
            let v: V = serde_json::from_slice(&data).unwrap();
            self.cache.insert(key, v.clone()).await;
            Some(v)
        } else {
            None
        }
    }

    #[tracing::instrument(skip(self, key, value))]
    pub async fn set<K>(&self, key: &K, value: &V)
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        self.cache.insert(key.clone(), value.clone()).await;
        let out = serde_json::to_vec(value).unwrap();
        if let Err(e) = self.client.put(&self.name, &key, out).await {
            error!("error setting cache: {:?}", e);
        }
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn remove<K>(&self, key: &K)
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        self.cache.remove(&key).await;
        if let Err(e) = self.client.delete(&self.name, &key).await {
            error!("error removing from cache: {:?}", e);
        }
    }
}
