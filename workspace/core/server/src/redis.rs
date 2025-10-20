use async_graphql::Error;
use redis::aio::{ConnectionManager, ConnectionManagerConfig, PubSub};
use redis::caching::CacheConfig;
use redis::{AsyncConnectionConfig, Client, ConnectionAddr, ConnectionInfo, ProtocolVersion, RedisConnectionInfo};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct RedisClient {
    connection: RedisConnection,
}

#[derive(Clone)]
pub struct RedisConnection {
    client: Client,
    manager: ConnectionManager,
}

impl std::fmt::Debug for RedisConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisConnection")
            .field("client", &self.client)
            .finish()
    }
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

    async fn new_cache_client(password: Option<String>, host: &String, port: u16) -> Result<Client, Error> {
        let info = ConnectionInfo {
            addr: ConnectionAddr::Tcp(host.to_string(), port),
            redis: RedisConnectionInfo {
                protocol: ProtocolVersion::RESP3,
                password: password.clone(),
                ..Default::default()
            },
        };
        let cache_config = CacheConfig::new().set_default_client_ttl(Duration::from_secs(60));
        let async_config = AsyncConnectionConfig::new().set_cache_config(cache_config);
        let client = Client::open(info)?;
        client.get_multiplexed_async_connection_with_config(&async_config).await?;
        Ok(client)
    }

    pub async fn new(host: String, port: u16, password: Option<String>) -> Result<Self, Error> {
        let cfg = ConnectionManagerConfig::new()
            .set_max_delay(30_000)
            .set_connection_timeout(Duration::from_millis(3000))
            .set_response_timeout(Duration::from_millis(3000));
        let password = password.unwrap_or_default();
        let client = Self::new_client(password.clone(), &host, port).await?;
        let mgr = ConnectionManager::new_with_config(client.clone(), cfg).await?;
        Ok(Self {
            connection: RedisConnection {
                client,
                manager: mgr,
            },
        })
    }

    pub async fn new_cache(host: String, port: u16, password: Option<String>) -> Result<Self, Error> {
        let cfg = ConnectionManagerConfig::new()
            .set_max_delay(30_000)
            .set_connection_timeout(Duration::from_millis(3000))
            .set_response_timeout(Duration::from_millis(3000))
            .set_cache_config(CacheConfig::new().set_default_client_ttl(Duration::from_secs(60)));
        let client = Self::new_cache_client(password.clone(), &host, port).await?;
        let mgr = ConnectionManager::new_with_config(client.clone(), cfg).await?;
        Ok(Self {
            connection: RedisConnection {
                client,
                manager: mgr,
            },
        })
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }

    pub async fn get_manager(&self) -> Result<ConnectionManager, Error> {
        let connection = self.get().await?;
        connection.get_connection().await
    }
}
