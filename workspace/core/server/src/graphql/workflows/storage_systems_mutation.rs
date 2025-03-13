use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::graphql::workflows::storage_system::StorageSystemObject;
use crate::models::workflow::storage_systems::StorageSystemInput;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct StorageSystemsMutationObject {}

#[Object(name = "StorageSystemsMutation")]
impl StorageSystemsMutationObject {

    async fn add(
        &self,
        ctx: &Context<'_>,
        storage_system: StorageSystemInput,
    ) -> Result<Option<StorageSystemObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.workflow.add_storage_system(ctx, &storage_system).await?;
        Ok(ctx.workflow.get_storage_system(&id).await?.map(StorageSystemObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        storage_system: StorageSystemInput,
    ) -> Result<Option<StorageSystemObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let id = Uuid::parse_str(&id)?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_storage_system(&id, &storage_system).await?;
        Ok(ctx.workflow.get_storage_system(&id).await?.map(StorageSystemObject::new))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let id = Uuid::parse_str(&id)?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.delete_storage_system(&id).await?;
        Ok(true)
    }
}
