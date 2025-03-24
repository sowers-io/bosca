use async_graphql::Error;
use log::info;
use redis::aio::{ConnectionManager, ConnectionManagerConfig, PubSub};
use redis::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisClient {
    connection: RedisConnection,
}

#[derive(Clone)]
pub struct RedisConnection {
    client: Client,
    manager: ConnectionManager,
}

impl RedisConnection {
    pub async fn get_connection(&self) -> Result<ConnectionManager, Error> {
        Ok(self.manager.clone())
    }

    pub async fn get_pubsub(&self) -> Result<PubSub, Error> {
        Ok(self.client.get_async_pubsub().await?)
    }
}

impl RedisClient {
    pub async fn new(url: String) -> Result<Self, Error> {
        info!("Connecting to Redis at {}", url);
        let cfg = ConnectionManagerConfig::new()
            .set_automatic_resubscription()
            .set_connection_timeout(Duration::from_millis(3000))
            .set_response_timeout(Duration::from_millis(3000));
        let mgr = ConnectionManager::new_with_config(Client::open(url.clone())?, cfg).await?;
        Ok(Self {
            connection: RedisConnection {
                client: Client::open(url.clone())?,
                manager: mgr,
            },
        })
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }
}
