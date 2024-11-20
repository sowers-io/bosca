use crate::graphql::workflows::model::ModelObject;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct ModelsObject {}

#[Object(name = "Models")]
impl ModelsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<ModelObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let models = ctx.workflow.get_models().await?;
        Ok(models.into_iter().map(ModelObject::new).collect())
    }

    async fn model(&self, ctx: &Context<'_>, id: String) -> Result<Option<ModelObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let uid = Uuid::from_str(id.as_str())?;
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow.get_model(&uid).await?.map(ModelObject::new))
    }
}
