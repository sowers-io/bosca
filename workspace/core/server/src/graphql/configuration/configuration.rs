use crate::context::BoscaContext;
use crate::graphql::configuration::configuration_permission::ConfigurationPermissionObject;
use crate::models::configuration::configuration::Configuration;
use crate::models::security::permission::PermissionAction;
use crate::security::evaluator::Evaluator;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct ConfigurationObject {
    configuration: Configuration,
}

impl ConfigurationObject {
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }
}

#[Object(name = "Configuration")]
impl ConfigurationObject {

    async fn id(&self) -> String {
        self.configuration.id.to_string()
    }

    async fn key(&self) -> &String {
        &self.configuration.key
    }

    async fn description(&self) -> &String {
        &self.configuration.description
    }

    async fn permissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ConfigurationPermissionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .configuration
            .get_permissions(&self.configuration.id)
            .await?
            .into_iter()
            .map(ConfigurationPermissionObject::new)
            .collect())
    }

    async fn value(&self, ctx: &Context<'_>) -> Result<Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission = ctx
            .configuration
            .get_permissions(&self.configuration.id)
            .await?;
        if !self.configuration.public {
            if permission.is_empty() {
                ctx.check_has_admin_account().await?;
            } else {
                let evaluator = Evaluator::new(self.configuration.id, permission);
                if !evaluator.evaluate(&ctx.principal, &PermissionAction::View) {
                    return Ok(None);
                }
            }
        }
        ctx
            .configuration
            .get_configuration_value(&self.configuration.key)
            .await
    }
}
