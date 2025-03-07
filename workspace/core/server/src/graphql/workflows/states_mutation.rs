use crate::graphql::workflows::state::WorkflowStateObject;
use crate::models::workflow::states::WorkflowStateInput;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub(crate) struct WorkflowStatesMutationObject {}

#[Object(name = "WorkflowStatesMutation")]
impl WorkflowStatesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        state: WorkflowStateInput,
    ) -> Result<Option<WorkflowStateObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.add_state(&state).await?;
        Ok(ctx.workflow
            .get_state(&state.id)
            .await?
            .map(WorkflowStateObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        state: WorkflowStateInput,
    ) -> Result<Option<WorkflowStateObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_state(&state).await?;
        Ok(ctx.workflow
            .get_state(&state.id)
            .await?
            .map(WorkflowStateObject::new))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.delete_state(&id).await?;
        Ok(true)
    }
}
