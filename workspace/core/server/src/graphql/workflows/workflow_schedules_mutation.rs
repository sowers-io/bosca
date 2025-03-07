use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::models::workflow::workflow_schedule::WorkflowScheduleInput;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::graphql::workflows::workflow_schedule::WorkflowScheduleObject;

pub(crate) struct WorkflowSchedulesMutationObject {}

#[Object(name = "WorkflowSchedulesMutation")]
impl WorkflowSchedulesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        collection_id: Option<String>,
        schedule: WorkflowScheduleInput,
    ) -> Result<Option<WorkflowScheduleObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = metadata_id.map(|id| id.parse::<uuid::Uuid>().unwrap());
        let collection_id = collection_id.map(|id| id.parse::<uuid::Uuid>().unwrap());
        let id = ctx.workflow_schedule.add(metadata_id, collection_id, &schedule).await?;
        let schedule = ctx.workflow_schedule.get(&id).await?;
        Ok(schedule.map(WorkflowScheduleObject::new))
    }

    async fn delete(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        ctx.workflow_schedule.delete(&id).await?;
        Ok(true)
    }
}
