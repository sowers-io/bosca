use async_graphql::Error;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::models::content::collection::CollectionType;
use crate::models::security::permission::PermissionAction;
use crate::util::storage::{storage_system_collection_delete, storage_system_metadata_delete};

pub async fn delete_collection(ctx: &BoscaContext, collection_id: &Uuid, recursive: Option<bool>) -> Result<(), Error> {
    let collection = ctx.check_collection_action(collection_id, PermissionAction::Delete).await?;
    if collection.collection_type == CollectionType::Root || collection.collection_type == CollectionType::System {
        return Err(Error::new("cannot delete root or system collection"));
    }
    if recursive.unwrap_or(false) {
        loop {
            let metadatas = ctx.content.get_collection_child_metadata(&collection, 0, 100).await?;
            if metadatas.is_empty() { break }
            for item in metadatas {
                ctx.content.remove_child_metadata(collection_id, &item.id).await?;
                let collection_ids = ctx.content.get_metadata_parent_collection_ids(&item.id, 0, 1).await?;
                if collection_ids.is_empty() {
                    delete_metadata(ctx, &item.id).await?;
                }
            }
        }
        loop {
            let collections = ctx.content.get_collection_child_collections(&collection, 0, 100).await?;
            if collections.is_empty() { break }
            for item in collections {
                Box::pin(delete_collection(ctx, &item.id, recursive)).await?;
            }
        }
    }
    if let Some(storage_system) = ctx.workflow
        .get_default_search_storage_system()
        .await? {
        storage_system_collection_delete(&collection, &storage_system, &ctx.search).await?;
    }
    ctx.content.delete_collection(collection_id).await?;
    Ok(())
}

pub async fn delete_metadata(ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
    let metadata = ctx.check_metadata_action(id, PermissionAction::Delete).await?;
    let storage_systems = ctx.workflow.get_storage_systems().await?;
    storage_system_metadata_delete(
        &ctx.storage,
        &metadata,
        &storage_systems,
        &ctx.search
    ).await?;
    let supplementaries = ctx.content.get_metadata_supplementaries(id).await?;
    for supplementary in supplementaries {
        let path = ctx.storage
            .get_metadata_path(&metadata, Some(supplementary.key.clone()))
            .await?;
        ctx.storage.delete(&path).await?;
    }
    // TODO: delete versions
    // TODO: delete search documents
    ctx.content.delete_metadata(id).await?;
    Ok(())
}