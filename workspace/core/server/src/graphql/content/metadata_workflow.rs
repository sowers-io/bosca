use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::metadata::Metadata;
use crate::models::workflow::execution_plan::WorkflowExecutionId;

pub struct MetadataWorkflowObject {
    pub metadata: Metadata,
}

#[Object(name = "MetadataWorkflow")]
impl MetadataWorkflowObject {
    async fn state(&self) -> &String {
        &self.metadata.workflow_state_id
    }

    async fn state_valid(&self) -> &Option<DateTime<Utc>> {
        &self.metadata.workflow_state_valid
    }

    async fn pending(&self) -> &Option<String> {
        &self.metadata.workflow_state_pending_id
    }

    async fn delete_workflow(&self) -> &Option<String> {
        &self.metadata.delete_workflow_id
    }

    async fn plans(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let plans_ids = ctx.content.metadata_workflows.get_metadata_plans(&self.metadata.id).await?;
        let mut plans = Vec::<WorkflowExecutionPlanObject>::new();
        for (plan_id, queue) in plans_ids {
            let id = WorkflowExecutionId {
                id: plan_id,
                queue,
            };
            let plan = ctx.workflow.get_execution_plan(&id).await?;
            if plan.is_none() {
                continue;
            }
            plans.push(plan.unwrap().into());
        }
        Ok(plans)
    }
}