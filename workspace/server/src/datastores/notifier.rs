use async_graphql::Error;
use futures_util::Stream;
use futures_util::StreamExt;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::redis::RedisClient;

pub struct Notifier {
    redis: RedisClient,
}

impl Notifier {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    pub async fn listen_metadata_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("metadata_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_collection_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("collection_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_workflow_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("workflow_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_activity_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("activity_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_trait_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("trait_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_storage_system_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("storage_system_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_model_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("model_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_prompt_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("prompt_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_state_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("state_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn metadata_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("metadata_changes", id)
            .await?;
        Ok(())
    }

    pub async fn collection_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("collection_changes", id)
            .await?;
        Ok(())
    }

    pub async fn workflow_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("workflow_changes", id)
            .await?;
        Ok(())
    }

    pub async fn activity_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("activity_changes", id)
            .await?;
        Ok(())
    }

    pub async fn trait_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("trait_changes", id)
            .await?;
        Ok(())
    }

    pub async fn storage_system_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("storage_system_changes", id)
            .await?;
        Ok(())
    }

    pub async fn model_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("model_changes", id)
            .await?;
        Ok(())
    }

    pub async fn prompt_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("prompt_changes", id)
            .await?;
        Ok(())
    }

    pub async fn state_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("state_changes", id)
            .await?;
        Ok(())
    }
}
