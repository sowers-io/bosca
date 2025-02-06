use crate::graphql::content::content_mutation::ContentMutationObject;
use crate::graphql::security::security_mutation::SecurityMutationObject;
use crate::graphql::workflows::workflows_mutation::WorkflowsMutationObject;
use async_graphql::Object;
use crate::graphql::profile::profiles_mutation::ProfilesMutationObject;
use crate::graphql::queries_mutation::PersistedQueriesMutationObject;

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

    async fn persisted_queries(&self) -> PersistedQueriesMutationObject {
        PersistedQueriesMutationObject {}
    }
}
