use crate::context::BoscaContext;
use crate::graphql::workflows::workflow::WorkflowObject;
use async_graphql::{Context, Error, Object};
use crate::models::content::template_attribute_workflow::TemplateAttributeWorkflow;

pub struct TemplateAttributeWorkflowObject {
    pub workflow: TemplateAttributeWorkflow,
}

impl TemplateAttributeWorkflowObject {
    pub fn new(workflow: TemplateAttributeWorkflow) -> Self {
        Self { workflow }
    }
}

#[Object(name = "TemplateAttributeWorkflow")]
impl TemplateAttributeWorkflowObject {
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
