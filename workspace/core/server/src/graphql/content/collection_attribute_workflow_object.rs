use crate::context::BoscaContext;
use crate::graphql::workflows::workflow::WorkflowObject;
use async_graphql::{Context, Error, Object};
use crate::models::content::collection_template_attribute_workflow::CollectionTemplateAttributeWorkflow;

pub struct CollectionTemplateAttributeWorkflowObject {
    pub workflow: CollectionTemplateAttributeWorkflow,
}

impl CollectionTemplateAttributeWorkflowObject {
    pub fn new(workflow: CollectionTemplateAttributeWorkflow) -> Self {
        Self { workflow }
    }
}

#[Object(name = "CollectionTemplateAttributeWorkflow")]
impl CollectionTemplateAttributeWorkflowObject {
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
