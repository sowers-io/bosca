use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::redis::RedisClient;
use async_graphql::{Error, SimpleObject};
use futures_util::Stream;
use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct Notifier {
    redis: RedisClient,
}

// TODO: check for access to the ID before forwarding on the event

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct SupplementaryIdObject {
    pub id: String,
    pub content_id: String,
    pub key: String,
    pub plan_id: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct TransitionIdObject {
    pub from_state_id: String,
    pub to_state_id: String,
}

impl Notifier {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    pub async fn listen_workflow_plan_finished(
        &self,
    ) -> Result<impl Stream<Item = WorkflowExecutionIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("workflow_plan_finished").await?;
        Ok(pubsub.into_on_message().filter_map(|msg| async move {
            let bytes = msg.get_payload_bytes();
            let publish: WorkflowExecutionId = serde_json::from_slice(bytes).ok()?;
            Some(WorkflowExecutionIdObject::new(publish))
        }))
    }

    pub async fn listen_workflow_plan_failed(
        &self,
    ) -> Result<impl Stream<Item = WorkflowExecutionIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("workflow_plan_failed").await?;
        Ok(pubsub.into_on_message().filter_map(|msg| async move {
            let bytes = msg.get_payload_bytes();
            let publish: WorkflowExecutionId = serde_json::from_slice(bytes).ok()?;
            Some(WorkflowExecutionIdObject::new(publish))
        }))
    }

    pub async fn listen_category_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("category_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_configuration_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("configuration_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_metadata_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("metadata_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_metadata_supplementary_changes(
        &self,
    ) -> Result<impl Stream<Item = SupplementaryIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("metadata_supplementary_changes").await?;
        Ok(pubsub.into_on_message().filter_map(|msg| async move {
            let bytes = msg.get_payload_bytes();
            let publish: SupplementaryIdObject = serde_json::from_slice(bytes).ok()?;
            Some(publish)
        }))
    }

    pub async fn listen_collection_supplementary_changes(
        &self,
    ) -> Result<impl Stream<Item = SupplementaryIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("collection_supplementary_changes").await?;
        Ok(pubsub.into_on_message().filter_map(|msg| async move {
            let bytes = msg.get_payload_bytes();
            let publish: SupplementaryIdObject = serde_json::from_slice(bytes).ok()?;
            Some(publish)
        }))
    }

    pub async fn listen_transition_changes(
        &self,
    ) -> Result<impl Stream<Item = TransitionIdObject>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("transition_changes").await?;
        Ok(pubsub.into_on_message().filter_map(|msg| async move {
            let bytes = msg.get_payload_bytes();
            let publish: TransitionIdObject = serde_json::from_slice(bytes).ok()?;
            Some(publish)
        }))
    }

    pub async fn listen_collection_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("collection_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_workflow_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("workflow_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    // pub async fn listen_workflow_activity_changes(&self) -> Result<impl Stream<Item = i64>, Error> {
    //     let connection = self.redis.get().await?;
    //     let mut pubsub = connection.get_pubsub().await?;
    //     pubsub.subscribe("workflow_activity_changes").await?;
    //     Ok(pubsub
    //         .into_on_message()
    //         .filter_map(|msg| async move { msg.get_payload().ok() }))
    // }

    pub async fn listen_workflow_schedule_changes(
        &self,
    ) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("workflow_schedule_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_activity_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("activity_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_trait_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("trait_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_storage_system_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("storage_system_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_model_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("model_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_prompt_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("prompt_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn listen_state_changes(&self) -> Result<impl Stream<Item = String>, Error> {
        let connection = self.redis.get().await?;
        let mut pubsub = connection.get_pubsub().await?;
        pubsub.subscribe("state_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }

    pub async fn transition_changed(
        &self,
        from_state_id: &str,
        to_state_id: &str,
    ) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = TransitionIdObject {
            from_state_id: from_state_id.to_string(),
            to_state_id: to_state_id.to_string(),
        };
        let data = serde_json::to_string(&id)?;
        conn.publish::<&str, String, ()>("transition_changes", data)
            .await?;
        Ok(())
    }

    pub async fn category_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("category_changes", id)
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

    pub async fn metadata_supplementary_changed(
        &self,
        supplementary_id: &Uuid,
        metadata_id: &Uuid,
        key: &str,
        plan_id: Option<String>,
    ) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let publish = SupplementaryIdObject {
            id: supplementary_id.to_string(),
            content_id: metadata_id.to_string(),
            key: key.to_string(),
            plan_id,
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

    pub async fn collection_supplementary_changed(
        &self,
        supplementary_id: &Uuid,
        collection_id: &Uuid,
        key: &str,
        plan_id: Option<Uuid>,
    ) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let publish = SupplementaryIdObject {
            id: supplementary_id.to_string(),
            content_id: collection_id.to_string(),
            key: key.to_string(),
            plan_id: plan_id.map(|id| id.to_string()),
        };
        let data = serde_json::to_string(&publish)?;
        conn.publish::<&str, String, ()>("collection_supplementary_changes", data)
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

    pub async fn workflow_activity_changed(&self, id: i64) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        conn.publish::<&str, i64, ()>("workflow_activity_changes", id)
            .await?;
        Ok(())
    }

    pub async fn workflow_schedule_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = id.to_string();
        conn.publish::<&str, String, ()>("workflow_schedule_changes", id)
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

    pub async fn workflow_plan_failed(&self, id: &WorkflowExecutionId) -> Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = serde_json::to_string(id)?;
        conn.publish::<&str, String, ()>("workflow_plan_failed", id)
            .await?;
        Ok(())
    }

    pub async fn workflow_plan_finished(&self, id: &WorkflowExecutionId) -> Result<(), Error> {
        let connection = self.redis.get().await?;
        let mut conn = connection.get_connection().await?;
        let id = serde_json::to_string(id)?;
        conn.publish::<&str, String, ()>("workflow_plan_finished", id)
            .await?;
        Ok(())
    }
}
