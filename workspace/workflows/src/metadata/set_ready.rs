use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::Value;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

pub struct MetadataSetReadyActivity {
    id: String,
}

impl Default for MetadataSetReadyActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataSetReadyActivity {
    pub fn new() -> MetadataSetReadyActivity {
        MetadataSetReadyActivity {
            id: "metadata.set.ready".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataSetReadyActivity {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Set Ready".to_string(),
            description: "Set metadata ready".to_string(),
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
        client.set_metadata_ready(&id).await?;
        Ok(())
    }
}
