use crate::queue::message::Message;
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use chrono::Utc;
use deadpool_postgres::Transaction;
use std::ops::Add;

pub struct JobCheckin {}

impl JobCheckin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobCheckin {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<Message>, Error> {
        for message in messages.iter_mut() {
            message.visible_timeout = Utc::now().add(chrono::Duration::minutes(15));
            queues
                .update_message_raw_txn(txn, queue, false, message)
                .await?;
        }
        Ok(messages.to_vec())
    }
}
