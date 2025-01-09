use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde::Deserialize;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_collection::CollectionInput;
use bosca_client::client::next_job::ActivityParameterType;

pub struct CollectionCreateActivity {
    id: String,
}

impl Default for CollectionCreateActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionCreateActivity {
    pub fn new() -> CollectionCreateActivity {
        CollectionCreateActivity {
            id: "collection.create".to_string(),
        }
    }
}

#[async_trait]
impl Activity for CollectionCreateActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, _: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let mut collection = CollectionInput::deserialize(&job.workflow_activity.configuration)?;
        if let Some(input) = job.activity.inputs.first() {
            if input.type_ == ActivityParameterType::CONTEXT {
                if let Some(value) = job.context.get(&input.name) {
                    collection.name = value.as_str().unwrap().to_string();
                }
            }
        }
        client.add_collection(collection).await?;
        Ok(())
    }
}
