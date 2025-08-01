use crate::context::{BoscaContext, PermissionCheck};
use crate::models::content::collection::CollectionType;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::{COLLECTION_DELETE_FINALIZE, METADATA_DELETE_FINALIZE};
use async_graphql::Error;
use uuid::Uuid;

pub async fn delete_collection(
    ctx: &BoscaContext,
    collection_id: &Uuid,
    recursive: Option<bool>,
    permanently: bool,
) -> Result<(), Error> {
    let check =
        PermissionCheck::new_with_collection_id(*collection_id, PermissionAction::Delete);
    let collection = ctx.collection_permission_check(check).await?;
    if (collection.collection_type == CollectionType::Root
        || collection.collection_type == CollectionType::System)
        && !ctx.has_admin_account().await?
    {
        return Err(Error::new("cannot delete root or system collection"));
    }
    if recursive.unwrap_or(false) {
        loop {
            let metadatas = ctx
                .content
                .collections
                .get_child_metadata(&collection, 0, 100)
                .await?;
            if metadatas.is_empty() {
                break;
            }
            for item in metadatas {
                ctx.content
                    .collections
                    .remove_child_metadata(ctx, collection_id, &item.id)
                    .await?;
                let collection_ids = ctx.content.metadata.get_parent_ids(&item.id, 0, 1).await?;
                if collection_ids.is_empty() {
                    if permanently {
                        ctx.content.metadata.delete(ctx, &item.id).await?;
                    } else {
                        ctx.content.metadata.mark_deleted(ctx, &item.id).await?;
                        let mut request = EnqueueRequest {
                            workflow_id: Some(METADATA_DELETE_FINALIZE.to_string()),
                            metadata_id: Some(item.id),
                            metadata_version: Some(item.version),
                            ..Default::default()
                        };
                        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
                    }
                }
            }
        }
        loop {
            let collections = ctx
                .content
                .collections
                .get_child_collections(&collection, 0, 100)
                .await?;
            if collections.is_empty() {
                break;
            }
            for item in collections {
                Box::pin(delete_collection(ctx, &item.id, recursive, permanently)).await?;
            }
        }
    }
    if permanently {
        ctx.content.collections.delete(ctx, collection_id).await?;
    } else {
        ctx.content
            .collections
            .mark_deleted(ctx, collection_id)
            .await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(COLLECTION_DELETE_FINALIZE.to_string()),
            collection_id: Some(*collection_id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
    }
    Ok(())
}
