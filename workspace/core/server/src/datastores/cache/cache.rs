use async_graphql::Error;
use async_nats::jetstream::kv::{Operation, Store};
use futures_util::StreamExt;
use log::{error, warn};
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
    cache: Arc<Cache<String, V>>,
    store: Store,
}

#[async_trait::async_trait]
pub trait ClearableCache {
    async fn clear(&self) -> Result<(), Error>;
    fn watch(&self);
}

#[async_trait::async_trait]
impl<V> ClearableCache for BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned + 'static,
{
    async fn clear(&self) -> Result<(), Error> {
        self.cache.invalidate_all();
        self.store.purge("*").await?;
        Ok(())
    }

    fn watch(&self) {
        let store = self.store.clone();
        let cache = Arc::clone(&self.cache);
        tokio::spawn(async move {
            loop {
                match store.watch("*").await {
                    Ok(mut stream) => {
                        while let Some(value) = stream.next().await {
                            if let Ok(value) = value {
                                let k = value.key;
                                match value.operation {
                                    Operation::Put => {
                                        let b = value.value;
                                        match serde_json::from_slice(&b) {
                                            Ok(v) => {
                                                cache.insert(k, v).await;
                                            }
                                            Err(e) => {
                                                error!("error parsing cache: {}", e);
                                            }
                                        }
                                    }
                                    Operation::Delete | Operation::Purge => {
                                        cache.remove(&k).await;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("error watching cache: {}", e);
                    }
                }
            }
        });
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
    pub fn new_ttl(_: String, size: u64, ttl: Duration, store: Store) -> Self {
        Self {
            cache: Arc::new(
                Cache::builder()
                    .max_capacity(size)
                    .time_to_idle(ttl)
                    .build(),
            ),
            store,
        }
    }

    pub async fn get<K>(&self, key: &K) -> Option<V>
    where
        K: Hash + Eq + ?Sized + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        let value = self.cache.get(&key).await;
        if value.is_some() {
            value
        } else {
            let out = self.store.get(key.clone()).await;
            match out {
                Ok(Some(v)) => {
                    let v: V = serde_json::from_slice(&v).unwrap();
                    self.cache.insert(key, v.clone()).await;
                    Some(v)
                }
                Ok(None) => None,
                Err(e) => {
                    warn!("error getting cache: {}", e);
                    None
                }
            }
        }
    }

    pub async fn set<K>(&self, key: &K, value: &V)
    where
        K: Hash + Eq + ?Sized + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        self.cache.insert(key.clone(), value.clone()).await;
        let out = serde_json::to_vec(value).unwrap();
        if let Err(e) = self.store.put(key, out.into()).await {
            error!("error setting cache: {}", e);
        }
    }

    pub async fn remove<K>(&self, key: &K)
    where
        K: Hash + Eq + ?Sized + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        self.cache.remove(&key).await;
        if let Err(e) = self.store.delete(key).await {
            error!("error removing from cache: {}", e);
        }
    }
}
