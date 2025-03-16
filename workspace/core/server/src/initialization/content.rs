use crate::context::BoscaContext;
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::Error;
use serde_json::Value;
use uuid::Uuid;

pub async fn initialize_content(ctx: &BoscaContext) -> Result<(), Error> {
    let root_collection_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;
    match ctx.content.collections.get(&root_collection_id).await? {
        Some(_) => {}
        None => {
            initialize_collection(ctx, "Root", CollectionType::Root, Value::Null).await?;
        }
    }
    Ok(())
}

async fn initialize_collection(
    ctx: &BoscaContext,
    name: &str,
    collection_type: CollectionType,
    attributes: Value,
) -> Result<(), Error> {
    let input = CollectionInput {
        parent_collection_id: None,
        name: name.to_string(),
        collection_type: Some(collection_type),
        attributes: if attributes.is_null() {
            None
        } else {
            Some(attributes)
        },
        ..Default::default()
    };
    let collection_id = ctx.content.collections.add(ctx, &input).await?;
    let group = ctx.security.get_administrators_group().await?;
    let permission = Permission {
        entity_id: collection_id,
        group_id: group.id,
        action: PermissionAction::Manage,
    };
    ctx.content.collection_permissions.add(&permission).await?;
    let principal = ctx.security.get_principal_by_identifier("admin").await?;
    let collection = ctx.content.collections.get(&collection_id).await?.unwrap();
    ctx.content
        .collection_workflows
        .set_state(
            ctx,
            &principal,
            &collection,
            "published",
            None,
            "initializing collections",
            true,
            true,
        )
        .await?;
    ctx.content
        .collection_workflows
        .set_ready(ctx, &collection_id)
        .await?;
    Ok(())
}
