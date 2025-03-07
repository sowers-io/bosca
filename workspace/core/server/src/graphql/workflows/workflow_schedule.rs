use crate::models::workflow::workflow_schedule::WorkflowSchedule;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::workflows::workflow::WorkflowObject;

pub struct WorkflowScheduleObject {
    schedule: WorkflowSchedule,
}

impl WorkflowScheduleObject {
    pub fn new(schedule: WorkflowSchedule) -> Self {
        Self { schedule }
    }
}

#[Object(name = "WorkflowSchedule")]
impl WorkflowScheduleObject {

    async fn id(&self) -> String {
        self.schedule.id.to_string()
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        if let Some(metadata_id) = &self.schedule.metadata_id {
            let ctx = ctx.data::<BoscaContext>()?;
            let metadata = ctx.content.metadata.get(metadata_id).await?;
            return Ok(metadata.map(MetadataObject::new));
        }
        Ok(None)
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<Option<CollectionObject>, Error> {
        if let Some(collection_id) = &self.schedule.collection_id {
            let ctx = ctx.data::<BoscaContext>()?;
            let collection = ctx.content.collections.get(collection_id).await?;
            return Ok(collection.map(CollectionObject::new));
        }
        Ok(None)
    }

    async fn workflow(&self, ctx: &Context<'_>) -> Result<Option<WorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let workflow = ctx.workflow.get_workflow(&self.schedule.workflow_id).await?;
        Ok(workflow.map(WorkflowObject::new))
    }

    async fn attributes(&self) -> &Option<Value> {
        &self.schedule.attributes
    }

    async fn configuration(&self) -> &Option<Value> {
        &self.schedule.configuration
    }

    async fn rrule(&self) -> String {
        self.schedule.rrule.to_string()
    }

    async fn starts(&self) -> &DateTime<Utc> {
        &self.schedule.starts
    }

    async fn ends(&self) -> &Option<DateTime<Utc>> {
        &self.schedule.ends
    }

    async fn last_run(&self) -> &Option<DateTime<Utc>> {
        &self.schedule.last_run
    }

    async fn next_run(&self) -> &Option<DateTime<Utc>> {
        &self.schedule.next_run
    }

    async fn last_scheduled(&self) -> &Option<DateTime<Utc>> {
        &self.schedule.last_scheduled
    }

    async fn enabled(&self) -> bool {
        self.schedule.enabled
    }
}

impl From<WorkflowSchedule> for WorkflowScheduleObject {
    fn from(schedule: WorkflowSchedule) -> Self {
        Self::new(schedule)
    }
}
