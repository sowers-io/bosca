use crate::context::BoscaContext;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::models::content::document_template_attribute_workflow::DocumentTemplateAttributeWorkflow;
use async_graphql::{Context, Error, Object};

pub struct DocumentTemplateAttributeWorkflowObject {
    pub workflow: DocumentTemplateAttributeWorkflow,
}

impl DocumentTemplateAttributeWorkflowObject {
    pub fn new(workflow: DocumentTemplateAttributeWorkflow) -> Self {
        Self { workflow }
    }
}

#[Object(name = "DocumentTemplateAttributeWorkflow")]
impl DocumentTemplateAttributeWorkflowObject {
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
