use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use deadpool_postgres::Transaction;
use serde_json::Value;

pub struct JobQueueSetContext {
    job_index: i32,
    context: Value,
}

impl JobQueueSetContext {
    pub fn new(context: &Value) -> Self {
        Self {
            job_index: -1,
            context: context.clone(),
        }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueSetContext {
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
                if self.job_index != -1 {
                    let job = plan.jobs.get_mut(self.job_index as usize).unwrap();
                    job.context = self.context.clone();
                } else {
                    plan.context = self.context.clone();
                }
            }
            MessageValue::Job(job) => {
                let set = JobQueueSetContext {
                    job_index: job.id.index,
                    context: self.context.clone(),
                };
                queues
                    .update_message(&job.execution_plan, false, &set)
                    .await?;
                job.context = self.context.clone();
            }
        }
        queues
            .update_message_raw_txn(txn, queue, false, message)
            .await?;
        Ok(messages.to_vec())
    }
}
