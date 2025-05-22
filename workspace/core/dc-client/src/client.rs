use crate::client::api::distributed_cache_client::DistributedCacheClient;
use crate::client::api::{
    ClearCacheRequest, CreateCacheRequest, DeleteValueRequest, Empty, GetValueRequest, Node,
    Notification, NotificationType, PutValueRequest, SubscribeNotificationsRequest,
};
use crate::client_watcher::watch;
use async_graphql::Error;
use hashring::HashRing;
use log::{error, info};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time::sleep;
use tonic::transport::channel::Change;
use tonic::transport::Channel;
use uuid::Uuid;

pub mod api {
    tonic::include_proto!("bosca.dc");
}

#[derive(Clone)]
pub struct Client {
    clients: Arc<RwLock<HashMap<String, DistributedCacheClient<Channel>>>>,
    hash: Arc<RwLock<HashRing<Node>>>,
    #[allow(dead_code)]
    origin_rx: Arc<broadcast::Receiver<Notification>>,
    tx: Arc<broadcast::Sender<Notification>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, s: &mut H) {
        (&self.id, &self.ip, self.port).hash(s)
    }
}

impl Eq for Node {}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel::<Notification>(500);
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            hash: Arc::new(RwLock::new(HashRing::new())),
            origin_rx: Arc::new(rx),
            tx: Arc::new(tx),
        }
    }

    async fn new_client(
        &self,
        host: String,
        port: u16,
    ) -> Result<DistributedCacheClient<Channel>, Error> {
        let (channel, sender) = Channel::balance_channel(1024);
        tokio::spawn(async move {
            loop {
                match std::env::var("KUBERNETES_NAMESPACE") {
                    Ok(namespace) => {
                        if let Err(e) = watch(namespace, port, &sender).await {
                            error!("Error watching kubernetes resources: {:?}", e);
                        }
                        sleep(Duration::from_secs(3)).await;
                    }
                    Err(_) => {
                        info!("Not running in kubernetes, using defined endpoints");
                        let url = format!("http://{}:{}", host, port);
                        let endpoint = tonic::transport::Endpoint::new(url)
                            .expect("invalid endpoint")
                            .connect_timeout(Duration::from_secs(3))
                            .timeout(Duration::from_secs(3))
                            .keep_alive_timeout(Duration::from_secs(3));
                        if let Err(e) = sender
                            .send(Change::Insert(host.to_string(), endpoint))
                            .await {
                            error!("failed to add endpoint: {}", e);
                        }
                        break;
                    }
                }
            }
        });
        Ok(DistributedCacheClient::new(channel))
    }

    async fn initialize_client(&self, node: Node) -> Result<(), Error> {
        let client = self.new_client(node.ip.clone(), node.port as u16).await?;
        let mut clients = self.clients.write().await;
        let mut hash = self.hash.write().await;
        clients.insert(node.id.clone(), client.clone());
        hash.add(node.clone());
        Ok(())
    }

    async fn destroy_client(&self, node: Node) {
        {
            let mut clients = self.clients.write().await;
            clients.remove(&node.id);
        }
        {
            let mut hash = self.hash.write().await;
            hash.remove(&node);
        }
    }

    async fn initialize_first_client(&self, host: String, port: u16) -> Result<(), Error> {
        let id = {
            let mut id = String::new();
            let mut client = self.new_client(host.clone(), port).await?;
            let nodes = client.get_nodes(Empty {}).await?;
            for node in &nodes.get_ref().nodes {
                self.initialize_client(node.clone()).await?;
                if id.is_empty() {
                    id = node.id.clone();
                }
            }
            id
        };
        if id.is_empty() {
            return Err(Error::new("no nodes available"));
        }
        let mut client = { self.clients.read().await.get(&id).unwrap().clone() };
        let listen_client = self.clone();
        tokio::spawn(async move {
            let Ok(mut response) = client
                .subscribe_notifications(SubscribeNotificationsRequest {})
                .await
            else {
                error!(
                    "subscribe notifications: failed to connect to {}:{}",
                    host, port
                );
                return;
            };
            let stream = response.get_mut();
            while let Ok(Some(notification)) = stream.message().await {
                let t = NotificationType::try_from(notification.notification_type).unwrap();
                match t {
                    NotificationType::CacheCreated => {
                        if let Err(e) = listen_client.tx.send(notification) {
                            error!("failed to send cache created notification: {}", e);
                        }
                    }
                    NotificationType::ValueUpdated => {
                        if let Err(e) = listen_client.tx.send(notification) {
                            error!("failed to send value updated notification: {}", e);
                        }
                    }
                    NotificationType::ValueDeleted => {
                        if let Err(e) = listen_client.tx.send(notification) {
                            error!("failed to send value deleted notification: {}", e);
                        }
                    }
                    NotificationType::CacheCleared => {
                        if let Err(e) = listen_client.tx.send(notification) {
                            error!("failed to send cache cleared notification: {}", e);
                        }
                    }
                    NotificationType::NodeFound => {
                        if let Some(node) = notification.node {
                            if let Err(e) = listen_client.initialize_client(node.clone()).await {
                                error!(
                                    "node found: failed to connect to {}:{}: {:?}",
                                    node.ip, node.port, e
                                );
                            }
                        }
                    }
                    NotificationType::NodeLost => {
                        if let Some(node) = notification.node {
                            listen_client.destroy_client(node).await;
                        }
                    }
                }
            }
        });
        Ok(())
    }

    #[tracing::instrument(skip(self, host, port))]
    pub async fn connect(&self, host: String, port: u16) -> Result<(), Error> {
        self.initialize_first_client(host, port).await?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Notification> {
        self.tx.subscribe()
    }

    #[tracing::instrument(skip(self, cache, max_capacity, ttl, tti))]
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

    pub async fn get_client(&self, key: &str) -> Result<DistributedCacheClient<Channel>, Error> {
        let hash = self.hash.read().await;
        if let Some(node) = hash.get(&key) {
            let clients = self.clients.read().await;
            if let Some(client) = clients.get(&node.id) {
                return Ok(client.clone());
            }
        }
        Err(Error::new("missing client"))
    }

    #[tracing::instrument(skip(self, cache, key))]
    pub async fn get(&self, cache: &str, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let mut client = self.get_client(key).await?;
        let request = GetValueRequest {
            cache: cache.to_string(),
            key: key.to_string(),
        };
        let value = client.get_value(request).await?;
        let r = value.get_ref();
        Ok(r.value.clone())
    }

    #[tracing::instrument(skip(self, cache, key, value))]
    pub async fn put(&self, cache: &str, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let mut client = self.get_client(key).await?;
        let request = PutValueRequest {
            request_id: Uuid::new_v4().to_string(),
            cache: cache.to_string(),
            key: key.to_string(),
            value,
        };
        client.put_value(request).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, cache, key))]
    pub async fn delete(&self, cache: &str, key: &str) -> Result<(), Error> {
        let mut client = self.get_client(key).await?;
        let request = DeleteValueRequest {
            cache: cache.to_string(),
            key: key.to_string(),
        };
        client.delete_value(request).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, cache))]
    pub async fn clear(&self, cache: &str) -> Result<(), Error> {
        let clients = self.clients.read().await;
        if let Some(client) = clients.values().next() {
            let mut client = client.clone();
            let request = ClearCacheRequest {
                cache: cache.to_string(),
            };
            client.clear_cache(request).await?;
        }
        Ok(())
    }
}
