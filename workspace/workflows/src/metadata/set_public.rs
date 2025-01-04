use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::add_collection::CollectionInput;
use bosca_client::client::enqueue_child_workflow::WorkflowConfigurationInput;
use bosca_client::client::get_collection_items::GetCollectionItemsContentCollectionItems;
use bosca_client::client::plan::ActivityParameterType;
use bosca_client::client::{enqueue_child_workflow, Client, WorkflowJob};
use serde::Deserialize;
use serde_json::Value;

pub struct MetadataSetPublicActivity {
    id: String,
}

impl Default for MetadataSetPublicActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataSetPublicActivity {
    pub fn new() -> MetadataSetPublicActivity {
        MetadataSetPublicActivity {
            id: "metadata.set.public".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataSetPublicActivity {
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
        client.set_metadata_public(&id, true).await?;
        Ok(())
    }
}
