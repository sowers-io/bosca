use crate::context::BoscaContext;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::util::profile::add_password_principal;
use async_graphql::Error;
use serde_json::Value;

pub async fn initialize_security(ctx: &BoscaContext) -> Result<(), Error> {
    match ctx.security.get_principal_by_identifier("admin").await {
        Ok(_) => {}
        Err(_) => {
            let groups = vec![];
            ctx.security
                .add_anonymous_principal(Value::Null, &groups)
                .await?;
            let identifier = "admin".to_string();
            let password = "password".to_string();
            let profile = ProfileInput {
                slug: None,
                name: "Administrator".to_string(),
                visibility: ProfileVisibility::Public,
                attributes: vec![],
            };
            let principal =
                add_password_principal(ctx, &identifier, &password, &profile, true, false).await?;
            let group = ctx.security.get_administrators_group().await?;
            ctx.security
                .add_principal_group(&principal.id, &group.id)
                .await?;
        }
    }
    Ok(())
}
