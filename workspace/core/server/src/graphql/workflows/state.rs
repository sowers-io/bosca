use crate::models::workflow::states::{WorkflowState, WorkflowStateType};
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::security::util::check_has_group;

pub struct WorkflowStateObject {
    state: WorkflowState,
}

impl WorkflowStateObject {
    pub fn new(state: WorkflowState) -> Self {
        Self { state }
    }
}

#[Object(name = "WorkflowState")]
impl WorkflowStateObject {
    async fn id(&self) -> String {
        self.state.id.to_string()
    }

    #[graphql(name = "type")]
    async fn state_type(&self) -> WorkflowStateType {
        self.state.state_type
    }

    async fn name(&self) -> &String {
        &self.state.name
    }

    async fn description(&self) -> &String {
        &self.state.description
    }

    async fn configuration(&self, ctx: &Context<'_>) -> Result<&Option<Value>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        Ok(&self.state.configuration)
    }

    async fn workflow_id(&self, ctx: &Context<'_>) -> Result<&Option<String>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        Ok(&self.state.workflow_id)
    }
    async fn entry_workflow_id(&self, ctx: &Context<'_>) -> Result<&Option<String>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        Ok(&self.state.entry_workflow_id)
    }
    async fn exit_workflow_id(&self, ctx: &Context<'_>) -> Result<&Option<String>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        Ok(&self.state.exit_workflow_id)
    }
}

impl From<WorkflowState> for WorkflowStateObject {
    fn from(state: WorkflowState) -> Self {
        Self::new(state)
    }
}
