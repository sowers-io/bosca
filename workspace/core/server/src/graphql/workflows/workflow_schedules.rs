use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_schedule::WorkflowScheduleObject;
use async_graphql::{Context, Error, Object};
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct WorkflowSchedulesObject {}

#[Object(name = "WorkflowSchedules")]
impl WorkflowSchedulesObject {

    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowScheduleObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let all = ctx.workflow_schedule.get_all().await?;
        Ok(all.into_iter().map(WorkflowScheduleObject::from).collect())
    }
}
