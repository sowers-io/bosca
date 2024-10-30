use crate::models::workflow::execution_plan::{WorkflowExecutionId, WorkflowJobId};
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use crate::worklfow::set_parent_complete::JobQueueSetParentComplete;
use async_graphql::Error;
use chrono::Utc;
use deadpool_postgres::Transaction;
use log::{info, warn};

pub struct JobQueueSetComplete {
    job_id: Option<WorkflowJobId>,
}

impl JobQueueSetComplete {
    pub fn new() -> Self {
        Self { job_id: None }
    }

    pub fn new_with_job(job_id: WorkflowJobId) -> Self {
        Self { job_id: Some(job_id) }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueSetComplete {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<Message>, Error> {
        if messages.is_empty() {
            return Ok(messages.to_vec());
        }
        for message in messages.iter_mut() {
            match &mut message.value {
                MessageValue::Plan(plan) => {
                    info!(target: "workflow", "setting plan complete: {}, {}", message.id, queue);
                    plan.error = None;
                    let mut update = true;
                    if let Some(index) = &self.job_id {
                        {
                            let current_job = plan.jobs.get_mut(index.index as usize).unwrap();
                            if current_job.children.len() == current_job.completed_children.len() {
                                current_job.complete = true;
                                current_job.error = None;
                            }
                        }
                        {
                            for job in plan.jobs.iter_mut() {
                                if job.complete {
                                    plan.pending.retain(|p| p.index != job.id.index);
                                    plan.running.retain(|p| p.index != job.id.index);
                                    plan.failed.retain(|p| p.index != job.id.index);
                                    plan.current.retain(|p| p.index != job.id.index);
                                }
                            }
                        }
                        {
                            if let Some(next) = &plan.next {
                                let next_job = plan.jobs.get(next.index as usize).unwrap();
                                if next_job.complete {
                                    plan.next = None;
                                }
                            }
                        }
                        let current_job = plan.jobs.get(index.index as usize).unwrap();
                        if current_job.complete {
                            plan.complete.insert(index.clone());
                            if plan.running.is_empty() && plan.current.is_empty() {
                                let next_execution_group = current_job.workflow_activity.execution_group + 1;
                                info!(target: "workflow", "no running jobs, checking for next group: {}, {} - execution group: {}", message.id, queue, next_execution_group);
                                let new_current: Vec<WorkflowJobId> = plan.jobs.iter().filter(|job| !job.complete && job.workflow_activity.execution_group == next_execution_group).map(|job| job.id.clone()).collect();
                                if new_current.is_empty() {
                                    update = false;
                                    plan.current.clear();
                                    info!(target: "workflow", "plan doesn't have any current jobs, archiving: {}", plan.plan_id);
                                    let current_plan_id = WorkflowExecutionId {
                                        id: message.id,
                                        queue: queue.to_owned(),
                                    };
                                    // fully archive plan
                                    queues.archive_txn(txn, &current_plan_id).await?;
                                    let parent_id = plan.parent.clone();
                                    queues.update_message_raw_txn(txn, queue, true, message).await?;
                                    if let Some(parent_id) = &parent_id {
                                        let executor = JobQueueSetParentComplete::new(current_plan_id.clone());
                                        queues.update_message_txn(txn, parent_id, false, &executor).await?;
                                    }
                                } else {
                                    info!(target: "workflow", "marking plan as ready for processing on the next execution group: {}, {}", message.id, queue);
                                    plan.current = new_current;
                                    message.visible_timeout = Utc::now();
                                }
                            } else {
                                info!(target: "workflow", "marking plan as ready for processing: {}, {}", message.id, queue);
                                message.visible_timeout = Utc::now();
                            }
                        } else {
                            update = false;
                            info!(target: "workflow", "not updating plan, waiting on child jobs: {}, {}", message.id, queue);
                        }
                    } else {
                        warn!(target: "workflow", "no job id supplied: {}, {}", message.id, queue);
                        return Err(Error::new(format!("no job id supplied: {}, {}", message.id, queue)));
                    }
                    if update {
                        queues.update_message_raw_txn(txn, queue, false, message).await?;
                    }
                }
                MessageValue::Job(job) => {
                    assert_ne!(job.execution_plan.id, 0, "execution plan id is zero");
                    info!(target: "workflow", "setting job complete: {}, {}", message.id, queue);
                    let set_complete = JobQueueSetComplete {
                        job_id: Some(WorkflowJobId {
                            id: message.id,
                            index: job.id.index,
                            queue: queue.to_owned(),
                        }),
                    };
                    let plans = queues.update_message_txn(txn, &job.execution_plan, false, &set_complete).await?;
                    let plan = match &plans.first().unwrap().value {
                        MessageValue::Plan(plan) => plan,
                        _ => return Err(Error::new("job queue update failed, invalid execution plan")),
                    };
                    let updated_job = plan.jobs.get(job.id.index as usize).unwrap();
                    if updated_job.children.len() == updated_job.completed_children.len() {
                        info!(target: "workflow", "archiving: {}, {}", message.id, queue);
                        let id = WorkflowExecutionId { id: message.id, queue: queue.to_owned() };
                        queues.archive_txn(txn, &id).await?
                    } else {
                        info!(target: "workflow", "not archiving, waiting on children: {}, {}", message.id, queue);
                    }
                }
            }
        }
        Ok(messages.to_owned())
    }
}
