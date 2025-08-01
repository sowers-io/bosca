use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};

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

    async fn running(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata_id(self.metadata.id.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        let running = ctx.workflow.get_metadata_count(&self.metadata.id).await?;
        Ok(running)
    }

    async fn plans(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata_id(self.metadata.id.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        let plans_ids = ctx
            .content
            .metadata_workflows
            .get_metadata_plans(&self.metadata.id)
            .await?;
        let mut plans = Vec::<WorkflowExecutionPlanObject>::new();
        for (plan_id, queue) in plans_ids {
            let id = WorkflowExecutionId { id: plan_id, queue };
            if let Ok(Some(plan)) = ctx.workflow.get_execution_plan(&id).await {
                plans.push(plan.into());
            }
        }
        Ok(plans)
    }
}
