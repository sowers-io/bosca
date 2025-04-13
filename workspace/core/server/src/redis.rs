use async_graphql::Error;
use redis::aio::{ConnectionManager, ConnectionManagerConfig, PubSub};
use redis::{Client, ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
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
    pub async fn new(host: String, port: u16, username: Option<String>, password: Option<String>) -> Result<Self, Error> {
        let info = ConnectionInfo {
            addr: ConnectionAddr::Tcp(host, port),
            redis: RedisConnectionInfo {
                username,
                password,
                ..Default::default()
            },
        };
        let cfg = ConnectionManagerConfig::new()
            .set_connection_timeout(Duration::from_millis(3000))
            .set_response_timeout(Duration::from_millis(3000));
        let mgr = ConnectionManager::new_with_config(Client::open(info.clone())?, cfg).await?;
        Ok(Self {
            connection: RedisConnection {
                client: Client::open(info.clone())?,
                manager: mgr,
            },
        })
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }
}
