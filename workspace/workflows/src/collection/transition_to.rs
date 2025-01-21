use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::Value;
use bosca_client::client::{get_collection_items, Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

pub struct CollectionTransitionToActivity {
    id: String,
}

impl Default for CollectionTransitionToActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionTransitionToActivity {
    pub fn new() -> CollectionTransitionToActivity {
        CollectionTransitionToActivity {
            id: "collection.transition.to".to_string(),
        }
    }
}

impl CollectionTransitionToActivity {

    #[allow(clippy::too_many_arguments)]
    async fn transition_children(&self, client: &Client, collection_id: &str, state: &str, status: &str, ready: bool, mark_public: bool, mark_public_list: bool) -> Result<(), Error> {
        const LIMIT: i64 = 100;
        let mut offset = 0;
        loop {
            let items = client.get_collection_items(collection_id, offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                match item {
                    get_collection_items::GetCollectionItemsContentCollectionItems::Collection(c) => {
                        self.transition_collection(client, &c.id, &c.workflow.state, &c.workflow.pending, state, status, ready && c.ready.is_none(), mark_public, mark_public_list).await?;
                        Box::pin(self.transition_children(client, &c.id, state, status, ready, mark_public, mark_public_list)).await?;
                    }
                    get_collection_items::GetCollectionItemsContentCollectionItems::Metadata(m) => {
                        self.transition_metadata(client, &m.id, &m.workflow.state, &m.workflow.pending, state, status, ready && m.ready.is_none(), mark_public).await?;
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn transition_collection(&self, client: &Client, id: &str, current_state: &str, pending_state: &Option<String>, state: &str, status: &str, ready: bool, mark_public: bool, mark_public_list: bool) -> Result<(), Error> {
        if pending_state.is_some() {
            client.set_collection_workflow_state_complete(id, status).await?;
        }
        if current_state != state {
            client.set_collection_workflow_state(id, state, status, true).await?;
        }
        if ready {
            client.set_collection_ready(id).await?
        }
        if mark_public {
            client.set_collection_public(id, true).await?;
        }
        if mark_public_list {
            client.set_collection_public_list(id, true).await?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn transition_metadata(&self, client: &Client, id: &str, current_state: &str, pending_state: &Option<String>, state: &str, status: &str, ready: bool, mark_public: bool) -> Result<(), Error> {
        if pending_state.is_some() {
            client.set_workflow_state_complete(id, status).await?;
        }
        if current_state != state {
            client.set_workflow_state(id, state, status, true).await?;
        }
        if ready {
            client.set_metadata_ready(id).await?
        }
        if mark_public {
            client.set_metadata_public(id, true).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl Activity for CollectionTransitionToActivity {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        let mut configuration = serde_json::Map::new();
        configuration.insert("state".to_string(), Value::String("draft".to_string()));
        configuration.insert("status".to_string(), Value::String("marked draft".to_string()));
        configuration.insert("ready".to_string(), Value::Bool(false));
        configuration.insert("mark_public".to_string(), Value::Bool(false));
        configuration.insert("mark_public_list".to_string(), Value::Bool(false));
        configuration.insert("recursive".to_string(), Value::Bool(false));
        ActivityInput {
            id: self.id.to_owned(),
            name: "Finalize Collection Transition".to_string(),
            description: "Finalize a Collections Transition".to_string(),
            child_workflow_id: None,
            configuration: Value::Object(configuration),
            inputs: vec![],
            outputs: vec![],
        }
    }

    async fn execute(&self, client: &Client, _: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let state = job.workflow_activity.configuration.get("state").unwrap().as_str().unwrap().to_owned();
        let status_value = job.workflow_activity.configuration.get("status").unwrap();
        let ready_value = job.workflow_activity.configuration.get("ready").unwrap_or(&Value::Null);
        let mark_public_value = job.workflow_activity.configuration.get("mark_public").unwrap_or(&Value::Null);
        let mark_public_list_value = job.workflow_activity.configuration.get("mark_public_list").unwrap_or(&Value::Null);
        let recursive_value = job.workflow_activity.configuration.get("recursive").unwrap_or(&Value::Null);
        let status = if status_value.is_null() {
            "".to_string()
        } else {
            status_value.as_str().unwrap().to_string()
        };
        let collection = &job.collection.clone().unwrap();
        if !recursive_value.is_null() && recursive_value.as_bool().unwrap() {
            self.transition_children(client, &collection.id, &state, &status, !ready_value.is_null() && ready_value.as_bool().unwrap(), !mark_public_value.is_null() && mark_public_value.as_bool().unwrap(), !mark_public_list_value.is_null() && mark_public_list_value.as_bool().unwrap()).await?;
        }
        self.transition_collection(client, &collection.id, &collection.workflow.state, &collection.workflow.pending, &state, &status, collection.ready.is_none() && !ready_value.is_null() && ready_value.as_bool().unwrap(), !mark_public_value.is_null() && mark_public_value.as_bool().unwrap(), !mark_public_list_value.is_null() && mark_public_list_value.as_bool().unwrap()).await?;
        Ok(())
    }
}
