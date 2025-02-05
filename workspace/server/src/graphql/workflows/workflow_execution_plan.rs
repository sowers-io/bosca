use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::graphql::workflows::workflow_job::WorkflowJobObject;
use crate::graphql::workflows::workflow_job_id::WorkflowJobIdObject;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

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
    async fn next(&self) -> Option<i32> {
        self.plan.next
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
        let metadata = ctx.check_metadata_action(&metadata_id, PermissionAction::View).await?;
        Ok(Some(MetadataObject::from(metadata)))
    }
    async fn metadata_version(&self) -> Option<i32> {
        self.plan.metadata_version
    }
    async fn collection_id(&self) -> Option<String> {
        self.plan.collection_id.as_ref().map(|id| id.to_string())
    }
    async fn supplementary_id(&self) -> &Option<String> {
        &self.plan.supplementary_id
    }
    async fn context(&self) -> &Value {
        &self.plan.context
    }
    async fn pending(&self) -> Vec<i32> {
        self.plan.pending.iter().cloned().collect()
    }
    async fn current_execution_group(&self) -> Vec<i32> {
        self.plan.current_execution_group.to_vec()
    }
    async fn running(&self) -> Vec<i32> {
        self.plan.running.iter().cloned().collect()
    }
    async fn complete(&self) -> Vec<i32> {
        self.plan.complete.iter().cloned().collect()
    }
    async fn failed(&self) -> Vec<i32> {
        self.plan.failed.iter().cloned().collect()
    }
    async fn error(&self) -> &Option<String> {
        &self.plan.error
    }
}

impl From<WorkflowExecutionPlan> for WorkflowExecutionPlanObject {
    fn from(plan: WorkflowExecutionPlan) -> Self {
        Self::new(plan)
    }
}
