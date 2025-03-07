use crate::graphql::workflows::storage_system::StorageSystemObject;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct StorageSystemsObject {}

#[Object(name = "StorageSystems")]
impl StorageSystemsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<StorageSystemObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let models = ctx.workflow.get_storage_systems().await?;
        Ok(models.into_iter().map(StorageSystemObject::new).collect())
    }

    async fn storage_system(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<StorageSystemObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let uid = Uuid::from_str(id.as_str())?;
        Ok(ctx.workflow
            .get_storage_system(&uid)
            .await?
            .map(StorageSystemObject::new))
    }
}
