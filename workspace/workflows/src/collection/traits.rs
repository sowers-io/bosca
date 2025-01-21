use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashSet;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

pub struct CollectionTraitsActivity {
    id: String,
}

impl Default for CollectionTraitsActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionTraitsActivity {
    pub fn new() -> CollectionTraitsActivity {
        CollectionTraitsActivity {
            id: "collection.traits.process".to_string(),
        }
    }
}

#[async_trait]
impl Activity for CollectionTraitsActivity {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Process Traits".to_string(),
            description: "Process a Collection's Traits".to_string(),
            child_workflow_id: None,
            configuration: Value::Null,
            inputs: vec![],
            outputs: vec![],
        }
    }

    async fn execute(&self, client: &Client, _: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        if job.collection.is_none() {
            return Err(Error::new("missing collection".to_string()));
        }
        let collection = job.collection.as_ref().unwrap();
        let mut executed: HashSet<String>;
        if job.context.is_null() {
            executed = HashSet::new();
        } else {
            let ctx = job.context.as_object().unwrap();
            let value = ctx.get("executed").unwrap_or(&Value::Null);
            if value.is_null() {
                executed = HashSet::new();
            } else {
                executed = value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();
            }
        }
        for trait_id in &collection.trait_ids {
            if executed.contains(trait_id) {
                continue;
            }
            let trait_ = client.get_trait(trait_id).await?;
            if trait_.is_none() {
                continue;
            }
            client
                .enqueue_child_workflows(&job.id.id, job.id.index, &job.id.queue, trait_.unwrap().workflow_ids)
                .await?;
            executed.insert(trait_id.clone());
            let updated_context = json!({"executed": executed.clone()});
            client
                .set_job_context(&job.id.id, job.id.index, &job.id.queue, &updated_context)
                .await?;
        }
        Ok(())
    }
}
