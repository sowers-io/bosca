use crate::context::BoscaContext;
use async_graphql::*;
use tokio_stream::Stream;
use crate::datastores::notifier::{SupplementaryIdObject, TransitionIdObject};
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;

pub struct SubscriptionObject;

#[Subscription(name = "Subscription")]
impl SubscriptionObject {

    async fn workflow_plan_failed(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = WorkflowExecutionIdObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_workflow_plan_failed().await
    }

    async fn workflow_plan_finished(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = WorkflowExecutionIdObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_workflow_plan_finished().await
    }

    async fn category(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_category_changes().await
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_metadata_changes().await
    }

    async fn metadata_supplementary(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = SupplementaryIdObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_metadata_supplementary_changes().await
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_collection_changes().await
    }

    async fn collection_supplementary(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = SupplementaryIdObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_collection_supplementary_changes().await
    }

    async fn workflow(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_workflow_changes().await
    }

    async fn workflow_schedule(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_workflow_schedule_changes().await
    }

    async fn activity(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_activity_changes().await
    }

    #[graphql(name = "trait")]
    async fn trait_(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_trait_changes().await
    }

    async fn storage_system(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_storage_system_changes().await
    }

    async fn model(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_model_changes().await
    }

    async fn prompt(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_prompt_changes().await
    }

    async fn state(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_state_changes().await
    }

    async fn configuration(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_configuration_changes().await
    }

    async fn transition(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = TransitionIdObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("Unauthorized"));
        }
        ctx.notifier.listen_transition_changes().await
    }
}
