use crate::models::workflow::execution_plan::{WorkflowExecutionId, WorkflowJobId};
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use chrono::{TimeDelta, Utc};
use deadpool_postgres::Transaction;
use std::ops::Add;

pub struct JobQueueFailer {
    job_id: Option<WorkflowJobId>,
    error: String,
}

impl JobQueueFailer {
    pub fn new(error: &str) -> Self {
        Self {
            job_id: None,
            error: error.to_owned(),
        }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueFailer {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<Message>, Error> {
        let message = messages.first_mut().unwrap();
        match &mut message.value {
            MessageValue::Plan(plan) => {
                if let Some(job_id) = &self.job_id {
                    plan.failed.insert(job_id.clone());
                    plan.running.remove(&job_id.clone());
                    if !plan.current.contains(job_id) {
                        plan.current.push(job_id.clone());
                    }
                    let job = plan.jobs.get_mut(job_id.index as usize).unwrap();
                    job.error = Some(self.error.clone());
                }
                plan.error = Some(self.error.clone());
            }
            MessageValue::Job(job) => {
                job.error = Some(self.error.clone());
                let failer = JobQueueFailer {
                    job_id: Some(job.id.clone()),
                    error: self.error.clone(),
                };
                let id = WorkflowExecutionId {
                    id: job.execution_plan.id,
                    queue: job.execution_plan.queue.clone(),
                };
                queues.update_message_txn(txn, &id, false, &failer).await?;
            }
        }
        message.visible_timeout = Utc::now().add(TimeDelta::seconds(30));
        queues
            .update_message_raw_txn(txn, queue, false, message)
            .await?;
        Ok(messages.to_vec())
    }
}
