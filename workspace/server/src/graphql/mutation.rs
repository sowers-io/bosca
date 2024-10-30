use crate::graphql::content::content_mutation::ContentMutationObject;
use crate::graphql::security::security_mutation::SecurityMutationObject;
use crate::graphql::workflows::workflows_mutation::WorkflowsMutationObject;
use async_graphql::Object;
use crate::graphql::queues::queues_mutation::QueuesMutationObject;

pub(crate) struct MutationObject;

#[Object(name = "Mutation")]
impl MutationObject {
    async fn content(&self) -> ContentMutationObject {
        ContentMutationObject {}
    }

    async fn workflows(&self) -> WorkflowsMutationObject {
        WorkflowsMutationObject {}
    }

    async fn security(&self) -> SecurityMutationObject {
        SecurityMutationObject {}
    }

    async fn queues(&self) -> QueuesMutationObject {
        QueuesMutationObject {}
    }
}
