use crate::context::BoscaContext;
use crate::security::firebase::{FirebaseImportUsers, HashConfig};
use async_graphql::*;

pub struct SecurityFirebaseMutationObject {}

#[Object(name = "SecurityFirebaseMutation")]
impl SecurityFirebaseMutationObject {

    async fn set_hash_config(&self, ctx: &Context<'_>, config: serde_json::Value) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let config: HashConfig = serde_json::from_value(config)?;
        ctx.security.set_firebase_hash_config(ctx, config).await?;
        Ok(true)
    }

    async fn import_users(&self, ctx: &Context<'_>, file: serde_json::Value) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let import: FirebaseImportUsers = serde_json::from_value(file)?;
        ctx.security.import_firebase_users(ctx, &import).await?;
        Ok(true)
    }
}
