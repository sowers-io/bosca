use crate::models::workflow::workflows::Workflow;
use async_graphql::Object;
use serde_json::Value;

pub struct WorkflowObject {
    workflow: Workflow,
}

impl WorkflowObject {
    pub fn new(workflow: Workflow) -> Self {
        Self { workflow }
    }
}

#[Object(name = "Workflow")]
impl WorkflowObject {
    async fn id(&self) -> String {
        self.workflow.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.workflow.name
    }

    async fn queue(&self) -> &String {
        &self.workflow.queue
    }

    async fn description(&self) -> &String {
        &self.workflow.description
    }

    async fn configuration(&self) -> &Value {
        &self.workflow.configuration
    }
}

impl From<Workflow> for WorkflowObject {
    fn from(workflow: Workflow) -> Self {
        Self::new(workflow)
    }
}
