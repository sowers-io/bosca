use crate::graphql::workflows::prompt::PromptObject;
use crate::models::workflow::activities::WorkflowActivityPrompt;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

pub struct WorkflowActivityPromptObject {
    prompt: WorkflowActivityPrompt,
}

impl WorkflowActivityPromptObject {
    pub fn new(prompt: WorkflowActivityPrompt) -> Self {
        Self { prompt }
    }
}

#[Object(name = "WorkflowActivityPrompt")]
impl WorkflowActivityPromptObject {
    async fn configuration(&self) -> &Option<Value> {
        &self.prompt.configuration
    }

    async fn prompt(&self, ctx: &Context<'_>) -> Result<PromptObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(PromptObject::new(
            ctx.workflow
                .get_prompt(&self.prompt.prompt_id)
                .await?
                .unwrap(),
        ))
    }
}

impl From<WorkflowActivityPrompt> for WorkflowActivityPromptObject {
    fn from(activity: WorkflowActivityPrompt) -> Self {
        Self::new(activity)
    }
}
