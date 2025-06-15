use crate::context::BoscaContext;
use crate::graphql::caches::CachesObject;
use crate::graphql::configuration::configurations::ConfigurationsObject;
use crate::graphql::content::content::ContentObject;
use crate::graphql::profiles::profiles::ProfilesObject;
use crate::graphql::queries::PersistedQueriesObject;
use crate::graphql::security::security::SecurityObject;
use crate::graphql::server::ServerObject;
use crate::graphql::workflows::workflows::WorkflowsObject;
use crate::models::content::search::{ SearchQuery, SearchResultObject };
use async_graphql::*;

pub struct QueryObject;

#[Object(name = "Query")]
impl QueryObject {
    async fn server(&self) -> ServerObject {
        ServerObject {}
    }

    async fn content(&self) -> ContentObject {
        ContentObject {}
    }

    async fn workflows(&self) -> WorkflowsObject {
        WorkflowsObject {}
    }

    async fn profiles(&self) -> ProfilesObject {
        ProfilesObject {}
    }

    async fn configurations(&self) -> ConfigurationsObject {
        ConfigurationsObject {}
    }

    async fn security(&self) -> SecurityObject {
        SecurityObject {}
    }

    async fn persisted_queries(&self) -> PersistedQueriesObject {
        PersistedQueriesObject {}
    }

    async fn caches(&self) -> CachesObject { CachesObject {} }

    async fn search(
        &self,
        ctx: &Context<'_>,
        query: SearchQuery,
    ) -> Result<SearchResultObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.search.search(ctx, &query).await
    }
}
