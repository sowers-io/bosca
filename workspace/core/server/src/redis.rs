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
    async fn new_client(password: String, host: &String, port: u16) -> Result<Client, Error> {
        let info = ConnectionInfo {
            addr: ConnectionAddr::Tcp(host.to_string(), port),
            redis: RedisConnectionInfo {
                password: Some(password.clone()),
                ..Default::default()
            },
        };
        let client = Client::open(info)?;
        // TODO: need to understand this, connection manager hangs if password is used, unless this happens.
        client.get_multiplexed_async_connection().await?;
        Ok(client)
    }

    pub async fn new(host: String, port: u16, password: Option<String>) -> Result<Self, Error> {
        let cfg = ConnectionManagerConfig::new()
            .set_max_delay(30_000)
            .set_connection_timeout(Duration::from_millis(3000))
            .set_response_timeout(Duration::from_millis(3000));
        let password = password.unwrap_or_default();
        let client = Self::new_client(password.clone(), &host, port).await?;
        let mgr = ConnectionManager::new_with_config(client, cfg).await?;
        Ok(Self {
            connection: RedisConnection {
                client: Self::new_client(password.clone(), &host, port).await?,
                manager: mgr,
            },
        })
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }
}
