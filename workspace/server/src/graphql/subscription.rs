use crate::context::BoscaContext;
use async_graphql::*;
use futures_util::StreamExt;
use tokio_stream::Stream;

pub struct SubscriptionObject;

#[Subscription(name = "Subscription")]
impl SubscriptionObject {

    async fn metadata(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        let mut pubsub = ctx.redis.get_async_pubsub().await?;
        pubsub.subscribe("metadata_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        let mut pubsub = ctx.redis.get_async_pubsub().await?;
        pubsub.subscribe("collection_changes").await?;
        Ok(pubsub
            .into_on_message()
            .filter_map(|msg| async move {
                msg.get_payload().ok()
            }))
    }
}
