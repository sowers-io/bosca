use crate::datastores::security::MODEL_MANAGERS_GROUP;
use crate::graphql::workflows::model::ModelObject;
use crate::models::workflow::models::ModelInput;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
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
        let group = ctx.security.get_workflow_manager_group().await?;
        if !ctx.principal.has_group(&group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let id = ctx.workflow.add_model(&model).await?;
        Ok(ctx.workflow.get_model(&id).await?.map(ModelObject::new))
    }
}
