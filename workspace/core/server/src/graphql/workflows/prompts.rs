use crate::graphql::workflows::prompt::PromptObject;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct PromptsObject {}

#[Object(name = "Prompts")]
impl PromptsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<PromptObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let models = ctx.workflow.get_prompts().await?;
        Ok(models.into_iter().map(PromptObject::new).collect())
    }

    async fn prompt(&self, ctx: &Context<'_>, id: String) -> Result<Option<PromptObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let uid = Uuid::from_str(id.as_str())?;
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow.get_prompt(&uid).await?.map(PromptObject::new))
    }
}
