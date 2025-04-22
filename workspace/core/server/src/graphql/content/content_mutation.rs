use crate::context::BoscaContext;
use crate::graphql::content::category_mutation::CategoryMutationObject;
use crate::graphql::content::collection_mutation::CollectionMutationObject;
use crate::graphql::content::metadata_mutation::MetadataMutationObject;
use crate::graphql::content::source_mutation::SourceMutationObject;
use async_graphql::{Context, Error, Object};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::REBUILD_STORAGE;

pub struct ContentMutationObject {}

#[Object(name = "ContentMutation")]
impl ContentMutationObject {
    async fn category(&self) -> CategoryMutationObject {
        CategoryMutationObject {}
    }
    async fn collection(&self) -> CollectionMutationObject {
        CollectionMutationObject {}
    }
    async fn metadata(&self) -> MetadataMutationObject {
        MetadataMutationObject {}
    }
    async fn sources(&self) -> SourceMutationObject {
        SourceMutationObject {}
    }

    async fn rebuild_storage_system_content(&self, ctx: &Context<'_>) -> async_graphql::Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal_groups.contains(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let mut request = EnqueueRequest {
            workflow_id: Some(REBUILD_STORAGE.to_string()),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(true)
    }
}
