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
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowExecutionId {
    pub queue: String,
    pub id: Uuid,
}

impl Display for WorkflowExecutionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorkflowExecutionId [ {}, {} ]", self.id, self.queue)
    }
}

#[derive(InputObject, Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowExecutionIdInput {
    pub queue: String,
    pub id: String,
}

impl From<WorkflowExecutionIdInput> for WorkflowExecutionId {
    fn from(value: WorkflowExecutionIdInput) -> Self {
        WorkflowExecutionId {
            id: Uuid::parse_str(&value.id).unwrap(),
            queue: value.queue,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct WorkflowJobId {
    pub queue: String,
    pub id: Uuid,
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
    pub id: String,
    pub index: i32,
}

impl From<WorkflowJobIdInput> for WorkflowJobId {
    fn from(value: WorkflowJobIdInput) -> Self {
        WorkflowJobId {
            id: Uuid::parse_str(&value.id).unwrap(),
            index: value.index,
            queue: value.queue,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionPlan {
    pub parent: Option<WorkflowJobId>,
    pub id: WorkflowExecutionId,
    pub enqueued: DateTime<Utc>,
    pub finished: Option<DateTime<Utc>>,
    pub workflow: Workflow,
    pub jobs: Vec<WorkflowJob>,
    pub metadata_id: Option<Uuid>,
    pub metadata_version: Option<i32>,
    pub collection_id: Option<Uuid>,
    pub supplementary_id: Option<String>,
    pub context: Value,
    pub next: Option<i32>,
    pub pending: HashSet<i32>,
    pub current_execution_group: Vec<i32>,
    pub running: HashSet<i32>,
    pub complete: HashSet<i32>,
    pub failed: HashSet<i32>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowJob {
    pub plan_id: WorkflowExecutionId,
    pub id: WorkflowJobId,
    pub workflow_id: String,
    pub collection_id: Option<String>,
    pub metadata_id: Option<String>,
    pub metadata_version: Option<i32>,
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
    pub finished: Option<DateTime<Utc>>,
}
