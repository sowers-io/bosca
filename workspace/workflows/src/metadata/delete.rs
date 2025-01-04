use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::enqueue_child_workflow::WorkflowConfigurationInput;
use bosca_client::client::get_collection_items::GetCollectionItemsContentCollectionItems;
use bosca_client::client::{Client, WorkflowJob};
use serde_json::Value;

pub struct MetadataDeleteActivity {
    id: String,
}

impl Default for MetadataDeleteActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataDeleteActivity {
    pub fn new() -> MetadataDeleteActivity {
        MetadataDeleteActivity {
            id: "metadata.delete".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataDeleteActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        _: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let id = if let Some(r) = job.workflow_activity.configuration.get("metadata_id") {
            r.as_str().unwrap_or("").to_owned()
        } else {
            job.metadata.as_ref().unwrap().id.to_owned()
        };
        client.delete_metadata(&id).await?;
        Ok(())
    }
}
