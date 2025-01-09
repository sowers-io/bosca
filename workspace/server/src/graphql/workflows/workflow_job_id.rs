use crate::models::workflow::execution_plan::WorkflowJobId;
use async_graphql::Object;

pub struct WorkflowJobIdObject {
    id: WorkflowJobId,
}

impl WorkflowJobIdObject {
    pub fn new(id: WorkflowJobId) -> Self {
        Self { id }
    }
}

#[Object(name = "WorkflowJobId")]
impl WorkflowJobIdObject {
    async fn queue(&self) -> &String {
        &self.id.queue
    }

    async fn id(&self) -> String {
        self.id.id.to_string()
    }

    async fn index(&self) -> i32 {
        self.id.index
    }
}

impl From<&WorkflowJobId> for WorkflowJobIdObject {
    fn from(id: &WorkflowJobId) -> Self {
        Self::new(id.clone())
    }
}
