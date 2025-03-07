use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::graphql::workflows::transition::TransitionObject;
use crate::security::util::check_has_group;
use async_graphql::*;

pub struct TransitionsObject {}

#[Object(name = "Transitions")]
impl TransitionsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<TransitionObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let transitions = ctx.workflow.get_transitions().await?;
        Ok(transitions.into_iter().map(TransitionObject::new).collect())
    }

    async fn transition(
        &self,
        ctx: &Context<'_>,
        from_state_id: String,
        to_state_id: String
    ) -> Result<Option<TransitionObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow
            .get_transition(&from_state_id, &to_state_id)
            .await?
            .map(TransitionObject::new))
    }
}
