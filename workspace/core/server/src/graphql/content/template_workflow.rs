use crate::context::BoscaContext;
use crate::graphql::workflows::workflow::WorkflowObject;
use async_graphql::{Context, Error, Object};
use crate::models::content::template_workflow::TemplateWorkflow;

pub struct TemplateWorkflowObject {
    pub workflow: TemplateWorkflow,
}

impl TemplateWorkflowObject {
    pub fn new(workflow: TemplateWorkflow) -> Self {
        Self { workflow }
    }
}

#[Object(name = "TemplateWorkflow")]
impl TemplateWorkflowObject {
    pub async fn auto_run(&self) -> bool {
        self.workflow.auto_run
    }

    pub async fn workflow(&self, ctx: &Context<'_>) -> Result<Option<WorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .workflow
            .get_workflow(&self.workflow.workflow_id)
            .await?
            .map(WorkflowObject::new))
    }
}
