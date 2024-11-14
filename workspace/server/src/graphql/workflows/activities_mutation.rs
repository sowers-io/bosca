use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::workflows::activity::ActivityObject;
use crate::models::workflow::activities::ActivityInput;

pub struct ActivitiesMutationObject {}

#[Object(name = "ActivitiesMutation")]
impl ActivitiesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        activity: ActivityInput,
    ) -> Result<Option<ActivityObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.add_activity(&activity).await?;
        let activity = ctx.workflow.get_activity(&activity.id).await?;
        if let Some(activity) = activity {
            let inputs = ctx.workflow.get_activity_inputs(&activity.id).await?;
            let outputs = ctx.workflow.get_activity_outputs(&activity.id).await?;
            Ok(Some(ActivityObject::new(&activity, Some(inputs), Some(outputs))))
        } else {
            Ok(None)
        }
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        activity: ActivityInput,
    ) -> Result<Option<ActivityObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_activity(&activity).await?;
        let activity = ctx.workflow.get_activity(&activity.id).await?;
        if let Some(activity) = activity {
            let inputs = ctx.workflow.get_activity_inputs(&activity.id).await?;
            let outputs = ctx.workflow.get_activity_outputs(&activity.id).await?;
            Ok(Some(ActivityObject::new(&activity, Some(inputs), Some(outputs))))
        } else {
            Ok(None)
        }
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        activity_id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.delete_activity(&activity_id).await?;
        Ok(true)
    }
}
