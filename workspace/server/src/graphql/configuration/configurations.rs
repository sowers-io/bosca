use crate::context::BoscaContext;
use async_graphql::*;
use crate::graphql::configuration::configuration::ConfigurationObject;
use crate::models::security::permission::PermissionAction;
use crate::security::evaluator::Evaluator;

pub struct ConfigurationsObject {}

#[Object(name = "Configurations")]
impl ConfigurationsObject {

    async fn all(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ConfigurationObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let all = ctx.configuration.get_configurations().await?;
        let mut cfg = Vec::new();
        for c in all {
            let permissions = ctx.configuration.get_permissions(&c.id).await?;
            if permissions.is_empty() {
                if !ctx.has_admin_account().await? {
                    continue;
                }
            }
            let evaluator = Evaluator::new(permissions);
            if evaluator.evaluate(&ctx.principal, &PermissionAction::List) {
                cfg.push(c);
            }
        }
        Ok(cfg.into_iter().map(ConfigurationObject::new).collect())
    }

    async fn get(
        &self,
        ctx: &Context<'_>,
        key: String,
    ) -> Result<Option<ConfigurationObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let configuration = ctx.configuration.get_configuration_by_key(&key).await?;
        if configuration.is_none() {
            return Ok(None);
        }
        let configuration = configuration.unwrap();
        let permissions = ctx.configuration.get_permissions(&configuration.id).await?;
        if permissions.is_empty() {
            ctx.check_has_admin_account().await?;
        }
        let evaluator = Evaluator::new(permissions);
        if evaluator.evaluate(&ctx.principal, &PermissionAction::View) {
            Ok(Some(ConfigurationObject::new(configuration)))
        } else {
            Ok(None)
        }
    }
}
