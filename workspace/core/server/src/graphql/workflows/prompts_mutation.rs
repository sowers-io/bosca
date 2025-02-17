use crate::datastores::security::MODEL_MANAGERS_GROUP;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::workflows::prompt::PromptObject;
use crate::models::workflow::prompts::PromptInput;

pub struct PromptsMutationObject {}

#[Object(name = "PromptsMutation")]
impl PromptsMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        prompt: PromptInput,
    ) -> Result<Option<PromptObject>, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.workflow.add_prompt(&prompt).await?;
        Ok(ctx.workflow.get_prompt(&id).await?.map(PromptObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        prompt: PromptInput,
    ) -> Result<Option<PromptObject>, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        ctx.workflow.edit_prompt(&id, &prompt).await?;
        Ok(ctx.workflow.get_prompt(&id).await?.map(PromptObject::new))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        ctx.workflow.delete_prompt(&id).await?;
        Ok(true)
    }
}
