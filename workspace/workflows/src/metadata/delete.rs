use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::Value;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

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

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Delete a Metadata".to_string(),
            description: "Delete a metadata and associated resources".to_string(),
            child_workflow_id: None,
            configuration: Value::Null,
            inputs: vec![],
            outputs: vec![],
        }
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
