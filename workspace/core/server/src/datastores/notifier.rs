use async_graphql::{Error, SimpleObject};
use futures_util::Stream;
use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::redis::RedisClient;

pub struct Notifier {
    redis: RedisClient,
}

// TODO: check for access to the ID before forwarding on the event

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct MetadataSupplementaryIdObject {
    pub id: String,
    pub supplementary: String,
}

impl Notifier {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    pub async fn listen_category_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("category_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    pub async fn listen_configuration_changes(&self) -> Result<impl Stream<Item=String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("configuration_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
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

    pub async fn listen_metadata_supplementary_changes(&self) -> Result<impl Stream<Item=MetadataSupplementaryIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("metadata_supplementary_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                let bytes = msg.get_payload_bytes();
                let publish: MetadataSupplementaryIdObject = serde_json::from_slice(&bytes).ok()?;
                Some(publish)
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

    pub async fn category_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("category_changed", id)
            .await?;
        Ok(())
    }

    pub async fn metadata_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("metadata_changes", id)
            .await?;
        Ok(())
    }

    pub async fn metadata_supplementary_changed(&self, id: &Uuid, key: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        let supplementary_id = key.to_string();
        let publish = MetadataSupplementaryIdObject {
            id,
            supplementary: supplementary_id,
        };
        let data = serde_json::to_string(&publish)?;
        conn.publish::<&str, String, ()>("metadata_supplementary_changes", data)
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

    pub async fn configuration_changed(&self, id: &str) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("configuration_changes", id)
            .await?;
        Ok(())
    }
}
