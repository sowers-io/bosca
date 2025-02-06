use crate::datastores::content::ContentDataStore;
use crate::datastores::profile::ProfileDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::profile::profile::ProfileInput;
use crate::models::security::credentials::PasswordCredential;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use async_graphql::Error;

pub async fn add_password_principal(
    security: &SecurityDataStore,
    content: &ContentDataStore,
    profiles: &ProfileDataStore,
    identifier: &str,
    password: &str,
    profile: &ProfileInput,
    auto_verify: bool,
) -> Result<Principal, Error> {
    let password_credential = PasswordCredential::new(identifier.to_string(), password.to_string());
    let groups = vec![];
    let principal_id = security
        .add_principal(
            auto_verify,
            serde_json::Value::Null,
            &password_credential,
            &groups,
        )
        .await?;

    let collection_name = format!("Collection for {}", identifier);
    let collection_id = content
        .add_collection(&CollectionInput {
            name: collection_name,
            collection_type: Some(CollectionType::System),
            ..Default::default()
        })
        .await?;

    let group_name = format!("principal.{}", principal_id);
    let description = format!("Group for {}", identifier);
    let group = security.add_group(&group_name, &description).await?;
    security
        .add_principal_group(&principal_id, &group.id)
        .await?;
    profiles
        .add_profile(&principal_id, &profile, &collection_id)
        .await?;
    let principal = security.get_principal_by_id(&principal_id).await?;

    let permission = Permission {
        entity_id: collection_id,
        group_id: group.id,
        action: PermissionAction::View,
    };
    content.add_collection_permission(&permission).await?;

    Ok(principal)
}
