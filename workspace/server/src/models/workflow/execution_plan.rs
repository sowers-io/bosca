use crate::models::workflow::activities::{
    Activity, ActivityParameter, WorkflowActivity, WorkflowActivityModel,
    WorkflowActivityParameter, WorkflowActivityPrompt, WorkflowActivityStorageSystem,
};
use crate::models::workflow::workflows::Workflow;
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowExecutionId {
    pub queue: String,
    pub id: i64,
}

impl Display for WorkflowExecutionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorkflowExecutionId [ {}, {} ]", self.id, self.queue)
    }
}

#[derive(InputObject, Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowExecutionIdInput {
    pub queue: String,
    pub id: i64,
}

impl From<WorkflowExecutionIdInput> for WorkflowExecutionId {
    fn from(value: WorkflowExecutionIdInput) -> Self {
        WorkflowExecutionId {
            id: value.id,
            queue: value.queue,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowJobId {
    pub queue: String,
    pub id: i64,
    pub index: i32,
}

impl Display for WorkflowJobId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WorkflowJobId [ {}, {}, {} ]",
            self.queue, self.id, self.index
        )
    }
}

#[derive(InputObject, Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowJobIdInput {
    pub queue: String,
    pub id: i64,
    pub index: i32,
}

impl From<WorkflowJobIdInput> for WorkflowJobId {
    fn from(value: WorkflowJobIdInput) -> Self {
        WorkflowJobId {
            id: value.id,
            index: value.index,
            queue: value.queue,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionPlan {
    pub parent: Option<WorkflowExecutionId>,
    pub plan_id: i64,
    pub enqueued: DateTime<Utc>,
    pub workflow: Workflow,
    pub jobs: Vec<WorkflowJob>,
    pub metadata_id: Option<String>,
    pub version: Option<i32>,
    pub collection_id: Option<String>,
    pub supplementary_id: Option<String>,
    pub context: Value,
    pub next: Option<WorkflowJobId>,
    pub pending: HashSet<WorkflowJobId>,
    pub current: Vec<WorkflowJobId>,
    pub running: HashSet<WorkflowJobId>,
    pub complete: HashSet<WorkflowJobId>,
    pub failed: HashSet<WorkflowJobId>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowJob {
    pub id: WorkflowJobId,
    pub execution_plan: WorkflowExecutionId,
    pub workflow_id: String,
    pub collection_id: Option<String>,
    pub metadata_id: Option<String>,
    pub version: Option<i32>,
    pub supplementary_id: Option<String>,
    pub activity: Activity,
    pub activity_inputs: Vec<ActivityParameter>,
    pub activity_outputs: Vec<ActivityParameter>,
    pub workflow_activity: WorkflowActivity,
    pub workflow_inputs: Vec<WorkflowActivityParameter>,
    pub workflow_outputs: Vec<WorkflowActivityParameter>,
    pub prompts: Vec<WorkflowActivityPrompt>,
    pub storage_systems: Vec<WorkflowActivityStorageSystem>,
    pub models: Vec<WorkflowActivityModel>,
    pub error: Option<String>,
    pub context: Value,
    pub children: HashSet<WorkflowExecutionId>,
    pub completed_children: HashSet<WorkflowExecutionId>,
    pub failed_children: HashSet<WorkflowExecutionId>,
    pub complete: bool,
}
