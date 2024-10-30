use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use deadpool_postgres::Transaction;

pub struct JobQueueGet {}

#[async_trait::async_trait]
impl MessageQueueExecutor<MessageValue> for JobQueueGet {
    async fn execute(
        &self,
        _: &MessageQueues,
        _: &Transaction<'_>,
        _: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<MessageValue>, Error> {
        for message in messages.as_mut_slice() {
            match &mut message.value {
                MessageValue::Plan(ref mut plan) => {
                    plan.plan_id = message.id;
                    for job in plan.jobs.as_mut_slice() {
                        job.execution_plan.id = message.id;
                    }
                }
                MessageValue::Job(ref mut job) => {
                    job.id.id = message.id;
                }
            }
        }
        Ok(messages.iter().map(|m| m.value.clone()).collect())
    }
}

pub struct JobQueueRawGet {}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueRawGet {
    async fn execute(
        &self,
        _: &MessageQueues,
        _: &Transaction<'_>,
        _: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<Message>, Error> {
        for message in messages.as_mut_slice() {
            match &mut message.value {
                MessageValue::Plan(ref mut plan) => {
                    plan.plan_id = message.id;
                }
                MessageValue::Job(ref mut job) => {
                    job.id.id = message.id;
                }
            }
        }
        Ok(messages.to_vec())
    }
}
