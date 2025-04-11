use crate::context::BoscaContext;
use async_graphql::*;
use crate::graphql::configuration::configuration::ConfigurationObject;
use crate::models::configuration::configuration::ConfigurationInput;
use crate::models::security::permission::PermissionAction;
use crate::security::evaluator::Evaluator;

pub struct ConfigurationsMutationObject {}

#[Object(name = "ConfigurationsMutation")]
impl ConfigurationsMutationObject {
    async fn set_configuration(
        &self,
        ctx: &Context<'_>,
        configuration: ConfigurationInput,
    ) -> Result<Option<ConfigurationObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = if let Some(cfg) = ctx.configuration.get_configuration_by_key(&configuration.key).await? {
            let permissions = ctx.configuration.get_permissions(&cfg.id).await?;
            let evaluator = Evaluator::new(cfg.id, permissions);
            if !evaluator.evaluate(&ctx.principal, &PermissionAction::Edit) {
                ctx.check_has_admin_account().await?;
            }
            ctx.configuration.set_configuration(&configuration).await?
        } else {
            ctx.check_has_admin_account().await?;
            ctx.configuration.set_configuration(&configuration).await?
        };
        if let Some(configuration) = ctx.configuration.get_configuration_by_id(&id).await? {
            Ok(Some(ConfigurationObject::new(configuration)))
        } else {
            Err(Error::new("failed to set configuration"))
        }
    }

    async fn delete_configuration(
        &self,
        ctx: &Context<'_>,
        key: String,
    ) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let cfg = ctx.configuration.get_configuration_by_key(&key).await?;
        if cfg.is_none() {
            return Ok(None);
        }
        let id = cfg.unwrap().id;
        let permissions = ctx.configuration.get_permissions(&id).await?;
        let evaluator = Evaluator::new(id, permissions);
        if !evaluator.evaluate(&ctx.principal, &PermissionAction::Delete) {
            ctx.check_has_admin_account().await?;
        }
        ctx.configuration.delete_configuration(&id).await?;
        Ok(Some(id.to_string()))
    }
}
