use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::MessageObject;

pub struct QueuesMutationObject {}

#[Object(name = "QueuesMutation")]
impl QueuesMutationObject {
    async fn retry(&self, ctx: &Context<'_>, queue: String, id: i64) -> Result<Option<MessageObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let id = WorkflowExecutionId {
            id,
            queue
        };
        Ok(ctx.messages.retry(&id).await?.map(MessageObject::new))
    }
}