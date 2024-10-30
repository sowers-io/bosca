use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};

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
