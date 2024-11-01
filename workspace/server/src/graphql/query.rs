use crate::graphql::content::content::ContentObject;
use crate::graphql::security::security::SecurityObject;
use crate::graphql::workflows::workflows::WorkflowsObject;
use async_graphql::*;
use crate::graphql::queues::queues::QueuesObject;

pub struct QueryObject;

#[Object(name = "Query")]
impl QueryObject {
    async fn content(&self) -> ContentObject {
        ContentObject {}
    }

    async fn workflows(&self) -> WorkflowsObject {
        WorkflowsObject {}
    }

    async fn security(&self) -> SecurityObject {
        SecurityObject {}
    }

    async fn queues(&self) -> QueuesObject {
        QueuesObject {}
    }
}
