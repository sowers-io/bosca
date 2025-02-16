use crate::models::workflow::states::{WorkflowState, WorkflowStateType};
use async_graphql::Object;
use serde_json::Value;

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

    async fn configuration(&self) -> &Option<Value> {
        &self.state.configuration
    }

    async fn workflow_id(&self) -> Option<&String> {
        self.state.workflow_id.as_ref()
    }
    async fn entry_workflow_id(&self) -> Option<&String> {
        self.state.entry_workflow_id.as_ref()
    }
    async fn exit_workflow_id(&self) -> Option<&String> {
        self.state.exit_workflow_id.as_ref()
    }
}

impl From<WorkflowState> for WorkflowStateObject {
    fn from(state: WorkflowState) -> Self {
        Self::new(state)
    }
}
