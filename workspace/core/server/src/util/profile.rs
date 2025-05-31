use crate::context::BoscaContext;
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::profiles::profile::ProfileInput;
use crate::models::security::credentials::Credential;
use crate::models::security::group_type::GroupType;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use async_graphql::Error;
use uuid::Uuid;

#[tracing::instrument(skip(ctx, credential, profile, verified, set_ready))]
pub async fn add_principal_with_credential(
    ctx: &BoscaContext,
    credential: &Credential,
    profile: &ProfileInput,
    verified: Option<bool>,
    set_ready: bool,
) -> Result<(Principal, Uuid), Error> {
    let groups = vec![];
    let principal_id = ctx
        .security
        .add_principal(
            verified,
            serde_json::Value::Null,
            &credential,
            &groups,
        )
        .await?;

    let group_name = format!("principal.{}", principal_id);
    let description = format!("Group for {}", principal_id);
    let group = ctx
        .security
        .add_group(&group_name, &description, GroupType::Principal)
        .await?;
    ctx.security
        .add_principal_group(&principal_id, &group.id)
        .await?;

    let collection_name = format!("Collection for {}", principal_id);
    let collection_id = ctx
        .content
        .collections
        .add(
            ctx,
            &CollectionInput {
                name: collection_name,
                collection_type: Some(CollectionType::System),
                trait_ids: Some(vec!["profile".to_string()]),
                ..Default::default()
            },
        )
        .await?;

    let profile = ctx
        .profile
        .add(ctx, Some(principal_id), profile, Some(collection_id))
        .await?;
    for action in [
        PermissionAction::View,
        PermissionAction::List,
        PermissionAction::Edit,
    ] {
        let permission = Permission {
            entity_id: collection_id,
            group_id: group.id,
            action,
        };
        ctx.content
            .collection_permissions
            .add(ctx, &permission)
            .await?;
    }

    let principal = ctx.security.get_principal_by_id(&principal_id).await?;
    if set_ready {
        let collection = ctx.content.collections.get(&collection_id).await?.expect("Collection not found");
        ctx.content
            .collection_workflows
            .set_ready_and_enqueue(ctx, &principal, &collection, None)
            .await?;
    }
    Ok((principal, profile))
}
