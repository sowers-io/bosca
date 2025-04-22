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
            if c.public {
                cfg.push(c);
                continue;
            }
            let permissions = ctx.configuration.get_permissions(&c.id).await?;
            if permissions.is_empty() && ctx.has_admin_account().await? {
                cfg.push(c);
                continue;
            }
            let evaluator = Evaluator::new(c.id, permissions);
            if evaluator.evaluate(&ctx.principal, &ctx.principal_groups, &PermissionAction::List) {
                cfg.push(c);
            }
        }
        Ok(cfg.into_iter().map(ConfigurationObject::new).collect())
    }

    async fn configuration(
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
        if configuration.public {
            return Ok(Some(ConfigurationObject::new(configuration)));
        }
        let permissions = ctx.configuration.get_permissions(&configuration.id).await?;
        if permissions.is_empty() {
            ctx.check_has_admin_account().await?;
            Ok(Some(ConfigurationObject::new(configuration)))
        } else {
            let evaluator = Evaluator::new(configuration.id, permissions);
            if evaluator.evaluate(&ctx.principal, &ctx.principal_groups, &PermissionAction::View) {
                Ok(Some(ConfigurationObject::new(configuration)))
            } else {
                Ok(None)
            }
        }
    }
}
