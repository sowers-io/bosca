use crate::graphql::workflows::state::WorkflowStateObject;
use crate::models::workflow::states::WorkflowStateInput;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;

pub(crate) struct WorkflowStatesMutationObject {}

#[Object(name = "WorkflowStatesMutation")]
impl WorkflowStatesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        state: WorkflowStateInput,
    ) -> Result<Option<WorkflowStateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let group = ctx.security.get_workflow_manager_group().await?;
        if !ctx.principal.has_group(&group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.workflow.add_state(&state).await?;
        Ok(ctx.workflow
            .get_state(&state.id)
            .await?
            .map(WorkflowStateObject::new))
    }
}
