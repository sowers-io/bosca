use async_graphql::Error;
use bosca_dc_client::client::Client;
use log::error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct BoscaCache<V>
where
    V: Clone + Send + Sync + serde::ser::Serialize + serde::de::DeserializeOwned,
{
    name: String,
    client: Client,
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
    pub fn new_ttl(name: String, client: Client) -> Self {
        Self { name, client, phantom_data: PhantomData }
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn get<K>(&self, key: &K) -> Option<V>
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
        if let Ok(Some(data)) = self.client.get(&self.name, &key).await {
            if let Ok(v) = serde_json::from_slice::<V>(&data) {
                return Some(v);
            }
        }
        None
    }

    #[tracing::instrument(skip(self, key, value))]
    pub async fn set<K>(&self, key: &K, value: &V)
    where
        K: Hash + Eq + Send + Sync + Display + Debug + Clone,
    {
        let key = key.to_string();
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
        if let Err(e) = self.client.delete(&self.name, &key).await {
            error!("error removing from cache: {:?}", e);
        }
    }
}
