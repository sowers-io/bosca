use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use deadpool_postgres::Transaction;
use log::{info, warn};
use crate::worklfow::set_complete::JobQueueSetComplete;

pub struct JobQueueSetParentComplete {
    workflow_id: WorkflowExecutionId,
    job_index: Option<i32>,
}

impl JobQueueSetParentComplete {
    pub fn new(workflow_id: WorkflowExecutionId) -> Self {
        Self {
            workflow_id,
            job_index: None,
        }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueSetParentComplete {
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
        for message in messages.as_mut_slice() {
            match &mut message.value {
                MessageValue::Plan(plan) => {
                    if let Some(job_index) = self.job_index {
                        info!(target: "workflow", "setting plan job children as complete: {}, {}", message.id, queue);
                        let job = plan.jobs.get_mut(job_index as usize).unwrap();
                        let completed_id = WorkflowExecutionId {
                            id: job.id.id,
                            queue: job.id.queue.clone(),
                        };
                        if let Some(parent) = &plan.parent {
                            let updater = JobQueueSetParentComplete {
                                job_index: Some(job_index),
                                workflow_id: completed_id,
                            };
                            queues.update_message_txn(txn, parent, false, &updater).await?;
                        }
                        job.completed_children.insert(self.workflow_id.clone());
                        queues.update_message_raw_txn(txn, queue, false, message).await?;
                    } else {
                        warn!(target: "workflow", "can't set plan job children as complete, missing job index: {}, {}", message.id, queue);
                    }
                }
                MessageValue::Job(job) => {
                    info!(target: "workflow", "setting job children as complete: {}, {}", message.id, queue);
                    job.completed_children.insert(self.workflow_id.clone());
                    let execution_plan = job.execution_plan.clone();
                    let parent_plan_completer = JobQueueSetParentComplete {
                        workflow_id: self.workflow_id.clone(),
                        job_index: Some(job.id.index),
                    };
                    let current_job_id = WorkflowExecutionId {
                        id: message.id,
                        queue: queue.to_owned(),
                    };
                    let complete = job.children.len() == job.completed_children.len();
                    queues.update_message_raw_txn(txn, queue, false, message).await?;
                    queues.update_message_txn(txn, &execution_plan, false, &parent_plan_completer).await?;
                    if complete {
                        let completer = JobQueueSetComplete::new();
                        queues.update_message_txn(txn, &current_job_id, false, &completer).await?;
                    }
                }
            }
        }
        Ok(messages.to_vec())
    }
}
