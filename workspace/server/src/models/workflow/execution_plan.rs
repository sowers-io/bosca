use crate::models::workflow::activities::{
    Activity, ActivityParameter, WorkflowActivity, WorkflowActivityModel,
    WorkflowActivityParameter, WorkflowActivityPrompt, WorkflowActivityStorageSystem,
};
use crate::models::workflow::workflows::Workflow;
use crate::worklfow::transaction::{RedisTransaction, RedisTransactionOp};
use async_graphql::{Error, InputObject};
use chrono::{DateTime, Utc};
use deadpool_postgres::Transaction;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use crate::worklfow::queue::WorkflowExecutionPlanResolver;

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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum WorkflowExecutePlanState {
    Running,
    Complete,
    Error,
}

impl WorkflowExecutePlanState {
    fn is_complete(&self) -> bool {
        self != &WorkflowExecutePlanState::Running
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum WorkflowExecuteJobState {
    NotComplete,
    Complete,
}

impl WorkflowExecutionPlan {
    fn try_set_next(&mut self) {
        if !self.running.is_empty() {
            return;
        }
        while !self.current_execution_group.is_empty() {
            info!(target: "workflow", "removing job from current list and queueing as next: {}", self.id);
            let next = self.current_execution_group.remove(0);
            if !self.complete.contains(&next) {
                self.next = Some(next);
                break;
            }
        }
    }

    async fn check_finished(
        &mut self,
        db_txn: &Transaction<'_>,
        redis_txn: &mut RedisTransaction,
        resolver: &WorkflowExecutionPlanResolver,
        next_execution_group: i32,
    ) -> Result<WorkflowExecutePlanState, Error> {
        if self.current_execution_group.is_empty() && self.running.is_empty() {
            self.current_execution_group = self.get_next_execution_group(next_execution_group);
            if self.current_execution_group.is_empty() {
                info!(target: "workflow", "plan doesn't have any current jobs, finishing: {}", self.id);
                self.finished = Some(Utc::now());
                if self.complete.len() != self.jobs.len() {
                    error!(target: "workflow", "plan finished job state is invalid: {}", self.id);
                    return Ok(WorkflowExecutePlanState::Error);
                }
                self.try_set_parent_complete(db_txn, redis_txn, resolver)
                    .await?;
                return Ok(WorkflowExecutePlanState::Complete);
            }
        }
        Ok(WorkflowExecutePlanState::Running)
    }

    pub async fn update(
        &mut self,
        db_txn: &Transaction<'_>,
        redis_txn: &mut RedisTransaction,
        resolver: &WorkflowExecutionPlanResolver,
        next_execution_group: i32,
    ) -> Result<WorkflowExecutePlanState, Error> {
        if self.next.is_some() && self.complete.contains(self.next.as_ref().unwrap()) {
            warn!("next is still defined, but is complete");
            self.next = None;
        }
        if self.next.is_none() {
            if next_execution_group == 1 {
                redis_txn.add_op(RedisTransactionOp::QueuePlan(self.id.clone()));
            }
            let state = self
                .check_finished(db_txn, redis_txn, resolver, next_execution_group)
                .await?;
            if state.is_complete() {
                return Ok(state);
            }
            self.try_set_next();
            let state = self
                .check_finished(db_txn, redis_txn, resolver, next_execution_group)
                .await?;
            if state.is_complete() {
                return Ok(state);
            }
        }
        if let Some(next) = self.next {
            let job = self.jobs.get(next as usize).unwrap();
            redis_txn.add_op(RedisTransactionOp::QueueJob(job.id.clone()));
        } else {
            if !self.running.is_empty() {
                return Err(Error::new("plan is still running jobs"));
            }
            error!(target: "workflow", "plan is missing next: {}", self.id);
        }
        if next_execution_group == 1 {
            if self.next.is_some() {
                resolver.set_plan(db_txn, self, true).await?;
            } else {
                return Ok(WorkflowExecutePlanState::Error);
            }
        }
        Ok(WorkflowExecutePlanState::Running)
    }

    async fn try_set_parent_complete(
        &mut self,
        db_txn: &Transaction<'_>,
        redis_txn: &mut RedisTransaction,
        resolver: &WorkflowExecutionPlanResolver,
    ) -> Result<(), Error> {
        if let Some(parent_id) = &self.parent {
            let Some(mut parent_plan) = resolver.get_plan(db_txn, parent_id).await? else {
                return Err(Error::new(
                    "can't mark execution complete, missing parent job",
                ));
            };
            let job = parent_plan.jobs.get_mut(parent_id.index as usize).unwrap();
            job.completed_children.insert(self.id.clone());
            Box::pin(parent_plan.try_set_job_complete(
                db_txn,
                redis_txn,
                resolver,
                parent_id)
            ).await?;
            resolver.set_plan(db_txn, &parent_plan, false).await?;
        }
        Ok(())
    }

    fn get_next_execution_group(&self, next_execution_group: i32) -> Vec<i32> {
        self.jobs
            .iter()
            .filter(|job| {
                !job.complete && job.workflow_activity.execution_group == next_execution_group
            })
            .map(|job| job.id.index)
            .collect()
    }

    pub async fn try_set_job_complete(
        &mut self,
        db_txn: &Transaction<'_>,
        redis_txn: &mut RedisTransaction,
        resolver: &WorkflowExecutionPlanResolver,
        job_id: &WorkflowJobId,
    ) -> Result<WorkflowExecutePlanState, Error> {
        redis_txn.add_op(RedisTransactionOp::RemoveJobRunning(job_id.clone()));
        let job = self.jobs.get_mut(job_id.index as usize).unwrap();
        job.error = None;
        job.complete = job.children.len() == job.completed_children.len();
        if job.complete {
            job.finished = Some(Utc::now());
            self.failed.remove(&job.id.index);
            self.running.remove(&job.id.index);
            self.complete.insert(job.id.index);
        }
        let next_execution_group = job.workflow_activity.execution_group + 1;
        Ok(self.update(db_txn, redis_txn, resolver, next_execution_group).await?)
    }

    pub async fn set_job_failed(
        &mut self,
        job_id: &WorkflowJobId,
        db_txn: &Transaction<'_>,
        redis_txn: &mut RedisTransaction,
        resolver: &WorkflowExecutionPlanResolver,
        error: &str,
    ) -> Result<(), Error> {
        self.failed.insert(job_id.index);
        self.running.remove(&job_id.index);
        if !self.current_execution_group.contains(&job_id.index) {
            self.current_execution_group.push(job_id.index);
        }
        let job = self.jobs.get_mut(job_id.index as usize).unwrap();
        job.error = Some(error.to_owned());
        resolver.set_plan(db_txn, self, false).await?;
        // TODO: setup exponential back-off and limit to failures
        redis_txn.add_op(RedisTransactionOp::RemoveJobRunning(job_id.clone()));
        redis_txn.add_op(RedisTransactionOp::QueueJob(job_id.clone()));
        Ok(())
    }
}
