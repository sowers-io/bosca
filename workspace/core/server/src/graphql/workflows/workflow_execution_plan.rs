use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::graphql::workflows::workflow_job::WorkflowJobObject;
use crate::graphql::workflows::workflow_job_id::WorkflowJobIdObject;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct WorkflowExecutionPlanObject {
    plan: WorkflowExecutionPlan,
}

impl WorkflowExecutionPlanObject {
    pub fn new(plan: WorkflowExecutionPlan) -> Self {
        Self { plan }
    }
}

#[Object(name = "WorkflowExecutionPlan")]
impl WorkflowExecutionPlanObject {
    async fn id(&self) -> WorkflowExecutionIdObject {
        WorkflowExecutionIdObject::new(self.plan.id.clone())
    }
    async fn parent(&self) -> Option<WorkflowJobIdObject> {
        let parent = self.plan.parent.clone();
        if parent.is_none() {
            None
        } else {
            parent.map(WorkflowJobIdObject::new)
        }
    }
    async fn workflow(&self) -> WorkflowObject {
        self.plan.workflow.clone().into()
    }
    async fn jobs(&self) -> Vec<WorkflowJobObject> {
        self.plan.jobs.iter().map(|j| j.clone().into()).collect()
    }
    async fn metadata_id(&self) -> Option<String> {
        self.plan.metadata_id.as_ref().map(|id| id.to_string())
    }
    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        if self.plan.metadata_id.is_none() {
            return Ok(None);
        }
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = self.plan.metadata_id.as_ref().unwrap();
        let check =
            PermissionCheck::new_with_metadata_id(*metadata_id, PermissionAction::View);
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(Some(MetadataObject::from(metadata)))
    }
    async fn metadata_version(&self) -> Option<i32> {
        self.plan.metadata_version
    }
    async fn collection_id(&self) -> Option<String> {
        self.plan.collection_id.as_ref().map(|id| id.to_string())
    }
    async fn enqueued(&self) -> &DateTime<Utc> {
        &self.plan.enqueued
    }
    async fn delayed_until(&self) -> &Option<DateTime<Utc>> {
        &self.plan.delay_until
    }
    async fn finished(&self) -> &Option<DateTime<Utc>> {
        &self.plan.finished
    }
    async fn failure(&self) -> bool {
        self.plan.failure
    }
    async fn supplementary_id(&self) -> &Option<String> {
        &self.plan.supplementary_id
    }
    async fn context(&self) -> &Option<Value> {
        &self.plan.context
    }
    async fn active(&self) -> Vec<i32> {
        self.plan.active.iter().cloned().collect()
    }
    async fn complete(&self) -> Vec<i32> {
        self.plan.complete.iter().cloned().collect()
    }
    async fn failed(&self) -> Vec<i32> {
        self.plan.failed.iter().cloned().collect()
    }
    async fn cancelled(&self) -> bool {
        self.plan.cancelled
    }
    async fn error(&self) -> &Option<String> {
        &self.plan.error
    }
    async fn max_failures(&self) -> i32 {
        self.plan.max_failures
    }
}

impl From<WorkflowExecutionPlan> for WorkflowExecutionPlanObject {
    fn from(plan: WorkflowExecutionPlan) -> Self {
        Self::new(plan)
    }
}
