use crate::context::BoscaContext;
use async_graphql::*;
use tokio_stream::Stream;

pub struct SubscriptionObject;

#[Subscription(name = "Subscription")]
impl SubscriptionObject {

    async fn metadata(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_metadata_changes().await
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_collection_changes().await
    }
}
