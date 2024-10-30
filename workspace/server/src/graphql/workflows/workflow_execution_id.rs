use crate::models::workflow::execution_plan::WorkflowExecutionId;
use async_graphql::Object;

pub struct WorkflowExecutionIdObject {
    id: WorkflowExecutionId,
}

impl WorkflowExecutionIdObject {
    pub fn new(id: WorkflowExecutionId) -> Self {
        Self { id }
    }
}

#[Object(name = "WorkflowExecutionId")]
impl WorkflowExecutionIdObject {
    async fn queue(&self) -> &String {
        &self.id.queue
    }

    async fn id(&self) -> i64 {
        self.id.id
    }
}

impl From<&WorkflowExecutionId> for WorkflowExecutionIdObject {
    fn from(id: &WorkflowExecutionId) -> Self {
        Self::new(id.clone())
    }
}
