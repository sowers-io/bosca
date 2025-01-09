use crate::models::workflow::execution_plan::{WorkflowExecutionPlan, WorkflowJob};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Serialize, Deserialize)]
pub enum JobQueueItem {
    Plan(WorkflowExecutionPlan),
    Job(WorkflowJob),
}
