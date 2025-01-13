use crate::models::workflow::workflows::Workflow;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_activity::WorkflowActivityObject;

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

    async fn activities(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowActivityObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let workflow_activities = ctx.workflow.get_workflow_activities(&self.workflow.id).await?;
        Ok(workflow_activities.iter().map(|a| WorkflowActivityObject::new(None, a)).collect())
    }
}

impl From<Workflow> for WorkflowObject {
    fn from(workflow: Workflow) -> Self {
        Self::new(workflow)
    }
}
