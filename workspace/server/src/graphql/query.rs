use crate::graphql::content::content::ContentObject;
use crate::graphql::queries::PersistedQueriesObject;
use crate::graphql::security::security::SecurityObject;
use crate::graphql::workflows::workflows::WorkflowsObject;
use async_graphql::*;
use crate::graphql::configuration::configurations::ConfigurationsObject;
use crate::graphql::profiles::profiles::ProfilesObject;

pub struct QueryObject;

#[Object(name = "Query")]
impl QueryObject {
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
}
