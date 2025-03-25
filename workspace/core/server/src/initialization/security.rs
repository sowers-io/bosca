use std::env;
use crate::context::BoscaContext;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::util::profile::add_password_principal;
use async_graphql::Error;
use log::info;
use serde_json::Value;

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
        Ok(admin_username) => admin_username,
        _ => {
            println!("Environment variable BOSCA_INIT_ADMIN_USERNAME could not be read, falling back to default value 'password'...");
            "admin".to_string()
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
            let principal =
                add_password_principal(ctx, &admin_username, &admin_password, &profile, true, false).await?;
            let group = ctx.security.get_administrators_group().await?;
            ctx.security
                .add_principal_group(&principal.id, &group.id)
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
            "admin".to_string()
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
            let principal =
                add_password_principal(ctx, &sa_username, &sa_password, &profile, true, false).await?;
            let group = ctx.security.get_service_account_group().await?;
            ctx.security
                .add_principal_group(&principal.id, &group.id)
                .await?;
            // TODO: relax this
            let group = ctx.security.get_administrators_group().await?;
            ctx.security
                .add_principal_group(&principal.id, &group.id)
                .await?;
        }
    }
    Ok(())
}
