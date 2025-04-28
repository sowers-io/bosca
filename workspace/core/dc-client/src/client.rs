use crate::client::api::distributed_cache_client::DistributedCacheClient;
use crate::client::api::{
    CreateCacheRequest, Node, Notification, NotificationType, SubscribeNotificationsRequest,
};
use async_graphql::Error;
use hashring::HashRing;
use log::error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tonic::transport::Channel;

mod api {
    tonic::include_proto!("bosca.dc");
}

#[derive(Clone)]
pub struct Client {
    clients: Arc<RwLock<HashMap<String, DistributedCacheClient<Channel>>>>,
    hash: Arc<RwLock<HashRing<Node>>>,
    tx: Arc<broadcast::Sender<Notification>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, s: &mut H) {
        (&self.id, &self.ip, self.port).hash(s)
    }
}

impl Eq for Node {}

impl Client {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<Notification>(500);
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            hash: Arc::new(RwLock::new(HashRing::new())),
            tx: Arc::new(tx),
        }
    }

    async fn initialize_client(
        &self,
        host: String,
        port: u16,
    ) -> Result<DistributedCacheClient<Channel>, Error> {
        let client = DistributedCacheClient::connect(format!("http://{}:{}", host, port)).await?;
        self.clients
            .write()
            .await
            .insert(host.clone(), client.clone());
        let mut subscribe_client = client.clone();
        let clients = Arc::clone(&self.clients);
        let hash = Arc::clone(&self.hash);
        tokio::spawn(async move {
            let Ok(mut response) = subscribe_client
                .subscribe_notifications(SubscribeNotificationsRequest {})
                .await
            else {
                error!("Failed to connect to {}:{}", host, port);
                return;
            };
            let stream = response.get_mut();
            while let Ok(Some(notification)) = stream.message().await {
                let t = NotificationType::try_from(notification.notification_type).unwrap();
                match t {
                    NotificationType::CacheCreated => {}
                    NotificationType::ValueUpdated => {}
                    NotificationType::ValueDeleted => {}
                    NotificationType::CacheCleared => {}
                    NotificationType::NodeFound => {}
                    NotificationType::NodeLost => {
                        let mut clients = clients.write().await;
                        let mut hash = hash.write().await;
                        if let Some(node) = notification.node {
                            clients.remove(&node.id);
                            hash.remove(&node);
                        }
                    }
                }
            }
        });
        Ok(client)
    }

    pub async fn connect(&self, host: String, port: u16) -> Result<(), Error> {
        self.initialize_client(host, port).await?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Notification> {
        self.tx.subscribe()
    }

    pub async fn create(
        &self,
        cache: &str,
        max_capacity: u64,
        ttl: u64,
        tti: u64,
    ) -> Result<(), Error> {
        let node = {
            let hash = self.hash.read().await;
            if let Some(node) = hash.get(&cache) {
                node.clone()
            } else {
                return Err(Error::new(format!("cache not available: {}", cache)));
            }
        };
        let request = CreateCacheRequest {
            name: cache.to_string(),
            max_capacity,
            ttl,
            tti,
        };
        if let Some(client) = self.clients.read().await.get(&node.id) {
            let mut client = client.clone();
            client.create_cache(request).await?;
        }

        Ok(())
    }

    pub async fn get(&self, cache: &str, key: &str) -> Result<Vec<u8>, Error> {
        todo!()
    }

    pub async fn put(&self, cache: &str, key: &str, value: Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    pub async fn delete(&self, cache: &str, key: &str) -> Result<(), Error> {
        todo!()
    }

    pub async fn clear(&self, cache: &str) -> Result<(), Error> {
        todo!()
    }
}
