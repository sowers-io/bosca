use async_graphql::Error;
use futures_util::Stream;
use futures_util::StreamExt;
use redis::{AsyncCommands, Client as RedisClient};
use uuid::Uuid;

pub struct ContentNotifier {
    redis: RedisClient,
}

impl ContentNotifier {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    pub async fn listen_metadata_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let mut pubsub = self.redis.get_async_pubsub().await?;
        pubsub.subscribe("metadata_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_collection_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let mut pubsub = self.redis.get_async_pubsub().await?;
        pubsub.subscribe("collection_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn metadata_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let mut conn = self.redis.get_multiplexed_tokio_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("metadata_changes", id)
            .await?;
        Ok(())
    }

    pub async fn collection_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let mut conn = self.redis.get_multiplexed_tokio_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("collection_changes", id)
            .await?;
        Ok(())
    }
}
