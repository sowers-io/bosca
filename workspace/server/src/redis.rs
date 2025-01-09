use async_graphql::Error;
use redis::aio::{ConnectionManager, PubSub};
use redis::Client;

#[derive(Clone)]
pub struct RedisClient {
    connection: RedisConnection,
}

#[derive(Clone)]
pub struct RedisConnection {
    client: Client,
    manager: ConnectionManager
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
        Ok(Self {
            connection: RedisConnection {
                client: Client::open(url.clone())?,
                manager: ConnectionManager::new(Client::open(url.clone())?).await?,
            },
        })
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }
}
