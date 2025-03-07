use crate::models::workflow::prompts::Prompt;
use async_graphql::Object;
use serde_json::Value;

pub struct PromptObject {
    prompt: Prompt,
}

impl PromptObject {
    pub fn new(prompt: Prompt) -> Self {
        Self { prompt }
    }
}

#[Object(name = "Prompt")]
impl PromptObject {
    async fn id(&self) -> String {
        self.prompt.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.prompt.name
    }

    async fn description(&self) -> &String {
        &self.prompt.description
    }

    async fn system_prompt(&self) -> &String {
        &self.prompt.system_prompt
    }

    async fn user_prompt(&self) -> &String {
        &self.prompt.user_prompt
    }

    async fn input_type(&self) -> &String {
        &self.prompt.input_type
    }

    async fn output_type(&self) -> &String {
        &self.prompt.output_type
    }

    async fn schema(&self) -> &Option<Value> {
        &self.prompt.schema
    }
}

impl From<Prompt> for PromptObject {
    fn from(prompt: Prompt) -> Self {
        Self::new(prompt)
    }
}
