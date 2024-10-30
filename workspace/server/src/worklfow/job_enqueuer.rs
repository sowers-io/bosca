use std::ops::Add;
use std::time::Duration;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use chrono::Utc;
use deadpool_postgres::Transaction;
use log::warn;
use crate::worklfow::set_complete::JobQueueSetComplete;

pub struct JobQueueEnqueuer {
    index: i32,
}

impl JobQueueEnqueuer {
    pub fn new(index: i32) -> Self {
        Self { index }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<WorkflowExecutionId> for JobQueueEnqueuer {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut jobs = Vec::<WorkflowExecutionId>::new();
        for message in messages {
            match &mut message.value {
                MessageValue::Plan(plan) => {
                    let job = plan.jobs.get_mut(self.index as usize).unwrap();
                    if job.id.id != 0 {
                        warn!(target: "workflow", "not enqueuing job, it already has a job id: {}", job.id);
                        if !queues.exists(&job.id.queue, job.id.id).await? {
                            let id = WorkflowExecutionId {
                              id: message.id,
                                queue: queue.to_owned(),
                            };
                            let completer = JobQueueSetComplete::new_with_job(job.id.clone());
                            queues.update_message_txn(txn, &id, false, &completer).await?;
                            continue;
                        }
                        message.visible_timeout = Utc::now().add(Duration::from_secs(3600));
                        queues
                            .update_message_raw_txn(txn, queue, false, message)
                            .await?;
                        continue;
                    }
                    if !plan.current.contains(&job.id)
                        && (plan.next.is_none()
                            || !plan.next.as_ref().is_some_and(|id| *id == job.id))
                    {
                        warn!(target: "workflow", "not enqueuing job, it's not marked as a current or next job: {}", job.id);
                        message.visible_timeout = Utc::now().add(Duration::from_secs(3600));
                        queues
                            .update_message_raw_txn(txn, queue, false, message)
                            .await?;
                        continue;
                    }
                    plan.next = None;
                    job.metadata_id = plan.metadata_id.clone();
                    job.execution_plan.id = message.id;
                    let value = MessageValue::Job(job.clone());
                    let enqueued_id = queues
                        .enqueue_txn(txn, &job.workflow_activity.queue, &value)
                        .await?;
                    let id = WorkflowExecutionId {
                        id: enqueued_id,
                        queue: job.workflow_activity.queue.clone(),
                    };
                    job.id.id = id.id;
                    plan.running.insert(job.id.clone());
                    queues
                        .update_message_raw_txn(txn, queue, false, message)
                        .await?;
                    jobs.push(id);
                }
                _ => panic!("unsupported"),
            }
        }
        Ok(jobs)
    }
}
