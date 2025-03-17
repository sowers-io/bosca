use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::profiles::profile::ProfileInput;
use crate::models::security::credentials::PasswordCredential;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use async_graphql::Error;
use crate::context::BoscaContext;

pub async fn add_password_principal(
    ctx: &BoscaContext,
    identifier: &str,
    password: &str,
    profile: &ProfileInput,
    auto_verify: bool,
    set_ready: bool
) -> Result<Principal, Error> {
    let password_credential = PasswordCredential::new(identifier.to_string(), password.to_string());
    let groups = vec![];
    let principal_id = ctx.security
        .add_principal(
            auto_verify,
            serde_json::Value::Null,
            &password_credential,
            &groups,
        )
        .await?;

    let collection_name = format!("Collection for {}", identifier);
    let collection_id = ctx.content
        .collections
        .add(ctx, &CollectionInput {
            name: collection_name,
            collection_type: Some(CollectionType::System),
            trait_ids: Some(vec!["profile".to_string()]),
            ..Default::default()
        })
        .await?;
    let collection = ctx.content.collections.get(&collection_id).await?.unwrap();

    let group_name = format!("principal.{}", principal_id);
    let description = format!("Group for {}", identifier);
    let group = ctx.security.add_group(&group_name, &description).await?;
    ctx.security
        .add_principal_group(&principal_id, &group.id)
        .await?;
    ctx.profile
        .add(ctx, Some(principal_id), profile, Some(collection_id))
        .await?;
    let principal = ctx.security.get_principal_by_id(&principal_id).await?;

    let permission = Permission {
        entity_id: collection_id,
        group_id: group.id,
        action: PermissionAction::View,
    };
    ctx.content.collection_permissions.add(&permission).await?;
    if set_ready {
        ctx.content.collection_workflows.set_ready_and_enqueue(ctx, &principal, &collection, None).await?;
    }

    Ok(principal)
}
