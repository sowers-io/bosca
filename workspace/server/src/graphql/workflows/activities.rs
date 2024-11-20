use crate::graphql::workflows::activity::ActivityObject;
use async_graphql::*;
use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct ActivitiesObject {}

#[Object(name = "Activities")]
impl ActivitiesObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<ActivityObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let mut activities = Vec::<ActivityObject>::new();
        for activity in ctx.workflow.get_activities().await? {
            activities.push(ActivityObject::new(&activity, None, None));
        }
        Ok(activities)
    }
}
