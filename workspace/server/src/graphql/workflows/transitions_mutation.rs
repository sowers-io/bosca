use crate::context::BoscaContext;
use crate::graphql::workflows::transition::TransitionObject;
use crate::models::workflow::transitions::TransitionInput;
use async_graphql::{Context, Error, Object};
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub(crate) struct TransitionsMutationObject {}

#[Object(name = "TransitionsMutation")]
impl TransitionsMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        transition: TransitionInput,
    ) -> Result<Option<TransitionObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.add_transition(&transition).await?;
        Ok(ctx
            .workflow
            .get_transition(&transition.from_state_id, &transition.to_state_id)
            .await?
            .map(TransitionObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        transition: TransitionInput,
    ) -> Result<Option<TransitionObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_transition(&transition).await?;
        Ok(ctx
            .workflow
            .get_transition(&transition.from_state_id, &transition.to_state_id)
            .await?
            .map(TransitionObject::new))
    }



    async fn delete(
        &self,
        ctx: &Context<'_>,
        from_state_id: String,
        to_state_id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow
            .delete_transition(&from_state_id, &to_state_id)
            .await?;
        Ok(true)
    }
}
