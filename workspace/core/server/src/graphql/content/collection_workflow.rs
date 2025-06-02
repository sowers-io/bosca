use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::collection::Collection;
use crate::models::workflow::execution_plan::WorkflowExecutionId;

pub struct CollectionWorkflowObject<'a> {
    pub collection: &'a Collection,
}

#[Object(name = "CollectionWorkflow")]
impl CollectionWorkflowObject<'_> {

    async fn state(&self) -> &String {
        &self.collection.workflow_state_id
    }

    async fn state_valid(&self) -> &Option<DateTime<Utc>> {
        &self.collection.workflow_state_valid
    }

    async fn pending(&self) -> &Option<String> {
        &self.collection.workflow_state_pending_id
    }

    async fn running(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.get_collection_count(&self.collection.id).await
    }

    async fn delete_workflow(&self) -> &Option<String> {
        &self.collection.delete_workflow_id
    }

    async fn plans(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let plans_ids = ctx.content.collection_workflows.get_plans(&self.collection.id).await?;
        let mut plans = Vec::<WorkflowExecutionPlanObject>::new();
        for (plan_id, queue) in plans_ids {
            let id = WorkflowExecutionId {
                id: plan_id,
                queue,
            };
            if let Ok(Some(plan)) = ctx.workflow.get_execution_plan(&id).await {
                plans.push(plan.into());
            }
        }
        Ok(plans)
    }
}
