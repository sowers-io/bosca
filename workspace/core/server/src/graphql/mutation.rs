use crate::graphql::configuration::configurations_mutation::ConfigurationsMutationObject;
use crate::graphql::content::content_mutation::ContentMutationObject;
use crate::graphql::profiles::profiles_mutation::ProfilesMutationObject;
use crate::graphql::queries_mutation::PersistedQueriesMutationObject;
use crate::graphql::security::security_mutation::SecurityMutationObject;
use crate::graphql::workflows::workflows_mutation::WorkflowsMutationObject;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;

pub(crate) struct MutationObject;

#[Object(name = "Mutation")]
impl MutationObject {
    async fn content(&self) -> ContentMutationObject {
        ContentMutationObject {}
    }

    async fn workflows(&self) -> WorkflowsMutationObject {
        WorkflowsMutationObject {}
    }

    async fn profiles(&self) -> ProfilesMutationObject {
        ProfilesMutationObject {}
    }

    async fn security(&self) -> SecurityMutationObject {
        SecurityMutationObject {}
    }

    async fn configurations(&self) -> ConfigurationsMutationObject {
        ConfigurationsMutationObject {}
    }

    async fn persisted_queries(&self) -> PersistedQueriesMutationObject {
        PersistedQueriesMutationObject {}
    }

    async fn clear_cache(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.cache.clear_all().await;
        Ok(true)
    }
}
