use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::graphql::workflows::workflow_job::WorkflowJobObject;
use crate::graphql::workflows::workflow_job_id::WorkflowJobIdObject;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;
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
    async fn id(&self) -> i64 {
        self.plan.plan_id
    }
    async fn parent(&self) -> Option<WorkflowExecutionIdObject> {
        let parent = self.plan.parent.clone();
        if parent.is_none() {
            None
        } else {
            parent.map(WorkflowExecutionIdObject::new)
        }
    }
    async fn workflow(&self) -> WorkflowObject {
        self.plan.workflow.clone().into()
    }
    async fn next(&self) -> Option<WorkflowJobIdObject> {
        let plan = &self.plan;
        plan.next.clone().map(WorkflowJobIdObject::new)
    }
    async fn jobs(&self) -> Vec<WorkflowJobObject> {
        self.plan.jobs.iter().map(|j| j.clone().into()).collect()
    }
    async fn metadata_id(&self) -> &Option<String> {
        &self.plan.metadata_id
    }
    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        if self.plan.metadata_id.is_none() {
            return Ok(None);
        }
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = self.plan.metadata_id.as_ref().unwrap();
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx.check_metadata_action(&id, PermissionAction::View).await?;
        Ok(Some(MetadataObject::from(metadata)))
    }
    async fn version(&self) -> Option<i32> {
        self.plan.version
    }
    async fn collection_id(&self) -> &Option<String> {
        &self.plan.collection_id
    }
    async fn supplementary_id(&self) -> &Option<String> {
        &self.plan.supplementary_id
    }
    async fn context(&self) -> &Value {
        &self.plan.context
    }
    async fn pending(&self) -> Vec<WorkflowJobIdObject> {
        self.plan
            .pending
            .iter()
            .map(WorkflowJobIdObject::from)
            .collect()
    }
    async fn current(&self) -> Vec<WorkflowJobIdObject> {
        self.plan
            .current
            .iter()
            .map(WorkflowJobIdObject::from)
            .collect()
    }
    async fn running(&self) -> Vec<WorkflowJobIdObject> {
        self.plan
            .running
            .iter()
            .map(WorkflowJobIdObject::from)
            .collect()
    }
    async fn complete(&self) -> Vec<WorkflowJobIdObject> {
        self.plan
            .complete
            .iter()
            .map(WorkflowJobIdObject::from)
            .collect()
    }
    async fn failed(&self) -> Vec<WorkflowJobIdObject> {
        self.plan
            .failed
            .iter()
            .map(WorkflowJobIdObject::from)
            .collect()
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
