use crate::datastores::security::MODEL_MANAGERS_GROUP;
use crate::graphql::workflows::model::ModelObject;
use crate::models::workflow::models::ModelInput;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::context::BoscaContext;

pub struct ModelsMutationObject {}

#[Object(name = "ModelsMutation")]
impl ModelsMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        model: ModelInput,
    ) -> Result<Option<ModelObject>, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.workflow.add_model(&model).await?;
        Ok(ctx.workflow.get_model(&id).await?.map(ModelObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        model: ModelInput,
    ) -> Result<Option<ModelObject>, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        ctx.workflow.edit_model(&id, &model).await?;
        Ok(ctx.workflow.get_model(&id).await?.map(ModelObject::new))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, MODEL_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        ctx.workflow.delete_model(&id).await?;
        Ok(true)
    }
}
