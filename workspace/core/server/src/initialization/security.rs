use crate::context::BoscaContext;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::security::credentials::Credential;
use crate::models::security::credentials_password::PasswordCredential;
use async_graphql::Error;
use log::info;
use serde_json::Value;
use std::env;

pub async fn initialize_security(ctx: &BoscaContext) -> Result<(), Error> {
    info!("Initialize Security");

    let admin_username = match env::var("BOSCA_INIT_ADMIN_USERNAME") {
        Ok(admin_username) => admin_username,
        _ => {
            println!("Environment variable BOSCA_INIT_ADMIN_USERNAME could not be read, falling back to default value 'admin'...");
            "admin".to_string()
        }
    };
    let admin_password = match env::var("BOSCA_INIT_ADMIN_PASSWORD") {
        Ok(admin_password) => admin_password,
        _ => {
            println!("Environment variable BOSCA_INIT_ADMIN_PASSWORD could not be read, falling back to default value 'password'...");
            "password".to_string()
        }
    };

    match ctx.security.get_principal_by_identifier(&admin_username).await {
        Ok(_) => {}
        Err(_) => {
            let groups = vec![];
            ctx.security
                .add_anonymous_principal(Value::Null, &groups)
                .await?;
            let profile = ProfileInput {
                slug: None,
                name: "Administrator".to_string(),
                visibility: ProfileVisibility::Public,
                attributes: vec![],
            };
            let credential = Credential::Password(PasswordCredential::new(admin_username, admin_password)?);
            let principal = ctx.security.
                add_principal_with_credential(ctx, &credential, &profile, Some(true), false, false).await?;
            let group = ctx.security.get_administrators_group().await?;
            ctx.security
                .add_principal_group(&principal.0, &group.id)
                .await?;
        }
    }

    let sa_username = match env::var("BOSCA_INIT_SA_USERNAME") {
        Ok(sa_username) => sa_username,
        _ => {
            println!("Environment variable BOSCA_INIT_SA_USERNAME could not be read, falling back to default value 'admin'...");
            "admin".to_string()
        }
    };
    let sa_password = match env::var("BOSCA_INIT_SA_PASSWORD") {
        Ok(sa_password) => sa_password,
        _ => {
            println!("Environment variable BOSCA_INIT_SA_PASSWORD could not be read, falling back to default value 'password'...");
            "password".to_string()
        }
    };

    match ctx.security.get_principal_by_identifier(&sa_username).await {
        Ok(_) => {}
        Err(_) => {
            let profile = ProfileInput {
                slug: None,
                name: "Service Account".to_string(),
                visibility: ProfileVisibility::Public,
                attributes: vec![],
            };
            let credential = Credential::Password(PasswordCredential::new(sa_username, sa_password)?);
            let principal = ctx.security.add_principal_with_credential(ctx, &credential, &profile, Some(true), false, false).await?;
            let group = ctx.security.get_service_account_group().await?;
            ctx.security
                .add_principal_group(&principal.0, &group.id)
                .await?;
            // TODO: relax this
            let group = ctx.security.get_administrators_group().await?;
            ctx.security
                .add_principal_group(&principal.0, &group.id)
                .await?;
        }
    }
    Ok(())
}
