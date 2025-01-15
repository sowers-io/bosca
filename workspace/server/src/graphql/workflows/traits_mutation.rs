use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::content::contenttrait::TraitObject;
use crate::models::workflow::traits::TraitInput;

pub struct TraitsMutationObject {}

#[Object(name = "TraitsMutation")]
impl TraitsMutationObject {

    async fn add(
        &self,
        ctx: &Context<'_>,
        model: TraitInput,
    ) -> Result<Option<TraitObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.add_trait(&model).await?;
        Ok(ctx.workflow.get_trait(&model.id).await?.map(TraitObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        model: TraitInput,
    ) -> Result<Option<TraitObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_trait(&model).await?;
        Ok(ctx.workflow.get_trait(&model.id).await?.map(TraitObject::new))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.delete_trait(&id).await?;
        Ok(true)
    }
}
