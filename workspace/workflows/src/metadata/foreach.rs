use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use serde_json::{from_str, json, Value};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::enqueue_child_workflow::WorkflowConfigurationInput;
use bosca_client::download::download_supplementary_path;

pub struct MetadataForEachActivity {
    id: String,
}

impl Default for MetadataForEachActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataForEachActivity {
    pub fn new() -> MetadataForEachActivity {
        MetadataForEachActivity {
            id: "metadata.foreach".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForEachWorkflows {
    pub workflows: Vec<ForEachWorkflow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForEachWorkflow {
    pub id: String,
    pub activities: Vec<ForEachWorkflowActivity>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForEachWorkflowActivity {
    pub id: String,
    pub configuration: Value
}

#[async_trait]
impl Activity for MetadataForEachActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata = job.metadata.as_ref().unwrap();
        let workflow_id = job.workflow_activity.configuration.get("workflow_id").unwrap().as_str().unwrap();
        let mut executed: HashSet<i64>;
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
                    .map(|v| v.as_i64().unwrap())
                    .collect();
            }
        }

        let input = &job.workflow_activity.inputs.first().unwrap().value;
        let download = client.get_metadata_supplementary_download(&metadata.id, input).await?.unwrap();
        let file = download_supplementary_path(&metadata.id, &download).await?;
        context.add_file_clean(&file);

        let mut f = File::open(&file).await?;
        let mut s = String::new();
        f.read_to_string(&mut s).await?;
        
        let workflows = from_str::<ForEachWorkflows>(&s)?;
        for (i, item) in workflows.workflows.iter().enumerate() {
            if executed.contains(&(i as i64)) {
                continue;
            }
            client
                .enqueue_child_workflow(job.id.id, &job.id.queue, workflow_id, item.activities.iter().map(|a| WorkflowConfigurationInput {
                    activity_id: a.id.to_owned(),
                    configuration: a.configuration.clone(),
                }).collect())
                .await?;
            executed.insert(i as i64);
            let updated_context = json!({"executed": executed.clone()});
            client
                .set_job_context(job.id.id, &job.id.queue, &updated_context)
                .await?;
        }
        Ok(())
    }
}
