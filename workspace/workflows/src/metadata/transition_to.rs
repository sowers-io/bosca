use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::Value;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

pub struct MetadataTransitionToActivity {
    id: String,
}

impl Default for MetadataTransitionToActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataTransitionToActivity {
    pub fn new() -> MetadataTransitionToActivity {
        MetadataTransitionToActivity {
            id: "metadata.transition.to".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataTransitionToActivity {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        let mut configuration = serde_json::Map::new();
        configuration.insert("state".to_string(), Value::String("draft".to_string()));
        configuration.insert("status".to_string(), Value::String("marked draft".to_string()));
        ActivityInput {
            id: self.id.to_owned(),
            name: "Finalize Metadata Transition".to_string(),
            description: "Finalize a Metadata Transition".to_string(),
            child_workflow_id: None,
            configuration: Value::Object(configuration),
            inputs: vec![],
            outputs: vec![],
        }
    }

    async fn execute(&self, client: &Client, _: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let state = job
            .workflow_activity
            .configuration
            .get("state")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let status_value = job.workflow_activity.configuration.get("status").unwrap();
        let status = if status_value.is_null() {
            "".to_string()
        } else {
            status_value.as_str().unwrap().to_string()
        };
        let metadata = &job.metadata.clone().unwrap();
        client
            .set_workflow_state_complete(&metadata.id, &status)
            .await?;
        client
            .set_workflow_state(&metadata.id, &state, &status, true)
            .await?;
        Ok(())
    }
}
