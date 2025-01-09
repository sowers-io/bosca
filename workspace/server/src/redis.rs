use async_graphql::Error;
use redis::Client;

#[derive(Clone)]
pub struct RedisClient {
    connection: RedisConnection,
}

#[derive(Clone)]
pub struct RedisConnection {
    client: Client
}

impl RedisConnection {

    pub async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, Error> {
        Ok(self.client.get_multiplexed_tokio_connection().await?)
    }

    pub async fn get_pubsub(&self) -> Result<redis::aio::PubSub, Error> {
        Ok(self.client.get_async_pubsub().await?)
    }
}

impl RedisClient {
    pub fn new(url: String) -> Self {
        Self {
            connection: RedisConnection { client: Client::open(url.clone()).unwrap() },
        }
    }

    pub async fn get(&self) -> Result<&RedisConnection, Error> {
        Ok(&self.connection)
    }
}
