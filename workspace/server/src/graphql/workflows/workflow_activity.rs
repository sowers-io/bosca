use crate::graphql::workflows::workflow_activity_parameter::WorkflowActivityParameterObject;
use crate::models::workflow::activities::WorkflowActivity;
use crate::models::workflow::execution_plan::WorkflowJob;
use async_graphql::Object;
use serde_json::Value;

pub struct WorkflowActivityObject {
    job: WorkflowJob,
    activity: WorkflowActivity,
}

impl WorkflowActivityObject {
    pub fn new(job: &WorkflowJob, activity: &WorkflowActivity) -> Self {
        Self {
            job: job.clone(),
            activity: activity.clone(),
        }
    }
}

#[Object(name = "WorkflowActivity")]
impl WorkflowActivityObject {
    async fn id(&self) -> i64 {
        self.activity.id
    }

    async fn queue(&self) -> &String {
        &self.activity.queue
    }

    async fn execution_group(&self) -> i32 {
        self.activity.execution_group
    }

    async fn configuration(&self) -> &Value {
        &self.activity.configuration
    }

    async fn inputs(&self) -> Vec<WorkflowActivityParameterObject> {
        self.job
            .workflow_inputs
            .iter()
            .map(|p| p.clone().into())
            .collect()
    }
    async fn outputs(&self) -> Vec<WorkflowActivityParameterObject> {
        self.job
            .workflow_outputs
            .iter()
            .map(|p| p.clone().into())
            .collect()
    }
}
