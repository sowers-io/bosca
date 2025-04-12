use crate::graphql::workflows::state::WorkflowStateObject;
use async_graphql::*;
use crate::context::BoscaContext;

pub struct WorkflowStatesObject {}

#[Object(name = "WorkflowStates")]
impl WorkflowStatesObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowStateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let states = ctx.workflow.get_states().await?;
        Ok(states.into_iter().map(WorkflowStateObject::new).collect())
    }

    async fn state(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<WorkflowStateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow
            .get_state(&id)
            .await?
            .map(WorkflowStateObject::new))
    }
}
