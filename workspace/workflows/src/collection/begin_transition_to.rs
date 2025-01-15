use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::begin_transition::BeginTransitionInput;
use bosca_client::client::{Client, WorkflowJob};

pub struct CollectionBeginTransitionToActivity {
    id: String,
}

impl Default for CollectionBeginTransitionToActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionBeginTransitionToActivity {
    pub fn new() -> CollectionBeginTransitionToActivity {
        CollectionBeginTransitionToActivity {
            id: "collection.begin.transition.to".to_string(),
        }
    }
}

#[async_trait]
impl Activity for CollectionBeginTransitionToActivity {
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
            .begin_transition(BeginTransitionInput {
                metadata_id: Some(metadata.id.to_owned()),
                version: Some(metadata.version.to_owned()),
                collection_id: None,
                state_id: state.to_owned(),
                status,
                supplementary_id: None,
                wait_for_completion: None
            })
            .await?;
        Ok(())
    }
}
