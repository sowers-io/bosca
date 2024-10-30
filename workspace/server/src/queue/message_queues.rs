use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::{Message, MessageValue};
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct MessageQueues {
    pool: Arc<Pool>,
}

#[async_trait::async_trait]
pub trait MessageQueueExecutor<T>: Send + Sync
where
    T: Send + Sync,
{
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<T>, Error>;
}

pub struct MessageQueueStats {
    pub size: i64,
    pub min: Option<DateTime<Utc>>,
    pub max: Option<DateTime<Utc>>,
    pub pending: i64,
    pub available: i64,
}

impl MessageQueues {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("CREATE EXTENSION IF NOT EXISTS pgmq")
            .await?;
        connection.execute(&stmt, &[]).await?;
        Ok(())
    }

    pub async fn create_queue(&self, name: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select pgmq.create($1)").await?;
        connection.execute(&stmt, &[name]).await?;
        Ok(())
    }

    pub async fn exists(&self, queue: &str, id: i64) -> Result<bool, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, queue, false).await?;
        let mut query = "select count(msg_id) as total from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id]).await?;
        let row = results.first().unwrap();
        let count: i64 = row.get("total");
        Ok(count > 0)
    }

    pub async fn get_queue_stats(&self, queue: &str, archived: bool) -> Result<MessageQueueStats, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, queue, archived).await?;
        let mut query = "select count(msg_id) as total, sum(case when vt > now() then 1 else 0 end) as pending, sum(case when vt < now() then 1 else 0 end) as available, min(enqueued_at) as min, max(enqueued_at) as max from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push('"');
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[]).await?;
        let row = results.first().unwrap();
        let available: Option<i64> = row.get("available");
        let pending: Option<i64> = row.get("pending");
        let min: Option<DateTime<Utc>> = row.get("min");
        let max: Option<DateTime<Utc>> = row.get("max");
        Ok(MessageQueueStats {
            size: row.get("total"),
            min,
            max,
            available: available.unwrap_or(0),
            pending: pending.unwrap_or(0),
        })
    }

    pub async fn get_messages_raw(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
        archived: bool,
    ) -> Result<Vec<Message>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" order by enqueued_at ");
        if archived {
            query.push_str("desc")
        } else {
            query.push_str("asc")
        }
        query.push_str(" offset $1 limit $2");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&offset, &limit]).await?;
        Ok(results.into_iter().map(|m| m.into()).collect())
    }

    pub async fn get_messages<T: Send + Sync>(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
        archived: bool,
        executor: &impl MessageQueueExecutor<T>,
    ) -> Result<Vec<T>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" order by enqueued_at ");
        if archived {
            query.push_str("desc")
        } else {
            query.push_str("asc")
        }
        query.push_str(" offset $1 limit $2");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&offset, &limit]).await?;
        let mut messages = results.into_iter().map(|m| m.into()).collect();
        let data = executor.execute(self, &txn, queue, &mut messages).await?;
        txn.commit().await?;
        Ok(data)
    }

    #[allow(dead_code)]
    pub async fn get_message_raw_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        id: &WorkflowExecutionId,
        archived: bool,
    ) -> Result<Option<Message>, Error> {
        let table_name = self.get_table_name_txn(txn, &id.queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        Ok(results.first().map(Message::from))
    }

    pub async fn get_message_raw(
        &self,
        id: &WorkflowExecutionId,
        archived: bool,
    ) -> Result<Option<Message>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        Ok(results.first().map(Message::from))
    }

    pub async fn get_message<T: Send + Sync>(
        &self,
        id: &WorkflowExecutionId,
        archived: bool,
        executor: &impl MessageQueueExecutor<T>,
    ) -> Result<Vec<T>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        let message = results.first().map(Message::from);
        if message.is_none() {
            return Ok(vec![]);
        }
        let mut messages = Vec::<Message>::new();
        messages.push(message.unwrap());
        let data = executor
            .execute(self, &txn, &id.queue, &mut messages)
            .await?;
        txn.commit().await?;
        Ok(data)
    }

    pub async fn archive_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        id: &WorkflowExecutionId,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare_cached("select pgmq.archive(queue_name => $1, msg_id => $2)")
            .await?;
        txn.execute(&stmt, &[&id.queue, &id.id]).await?;
        Ok(())
    }

    // async fn poll_for_message<T>(
    //     &self,
    //     txn: &Transaction<'_>,
    //     queue: &String,
    //     timeout: i32,
    //     count: i32,
    //     tx: &UnboundedSender<Vec<T>>,
    //     executor: &impl MessageQueueExecutor<T>,
    // ) -> Result<bool, Error>
    // where
    //     T: Send + Sync,
    // {
    //     let stmt = txn
    //         .prepare_cached("select msg_id, message, enqueued_at, vt from pgmq.read($1, $2, $3)")
    //         .await?;
    //     let results = txn.query(&stmt, &[&queue, &timeout, &count]).await?;
    //     let mut messages: Vec<Message> = results.into_iter().map(|m| m.into()).collect();
    //     let result = executor.execute(self, txn, queue, &mut messages).await?;
    //     tx.send(result)?;
    //     Ok(true)
    // }

    pub async fn enqueue(&self, queue: &str, message: &MessageValue) -> Result<i64, Error> {
        let value = serde_json::to_value(message)?;
        let connection = self.pool.get().await?;
        let queue = queue.to_owned();
        let stmt = connection
            .prepare("select pgmq.send(queue_name => $1, msg => $2)")
            .await?;
        let row = connection.query_one(&stmt, &[&queue, &value]).await?;
        Ok(row.get(0))
    }

    pub async fn enqueue_with_executor(
        &self,
        id: &WorkflowExecutionId,
        executor: &impl MessageQueueExecutor<WorkflowExecutionId>,
    ) -> Result<Option<WorkflowExecutionId>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, false).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        let mut messages: Vec<Message> = results.into_iter().map(|m| m.into()).collect();
        if messages.is_empty() {
            return Ok(None);
        }
        let new_messages = executor
            .execute(self, &txn, &id.queue, &mut messages)
            .await?;
        txn.commit().await?;
        Ok(new_messages.first().cloned())
    }

    pub async fn enqueue_multi_with_executor(
        &self,
        id: &WorkflowExecutionId,
        executor: &impl MessageQueueExecutor<WorkflowExecutionId>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, false).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        let mut messages: Vec<Message> = results.into_iter().map(|m| m.into()).collect();
        if messages.is_empty() {
            return Ok(Vec::new());
        }
        let new_messages = executor
            .execute(self, &txn, &id.queue, &mut messages)
            .await?;
        txn.commit().await?;
        Ok(new_messages)
    }

    pub async fn enqueue_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        queue: &String,
        message: &MessageValue,
    ) -> Result<i64, Error> {
        let value = serde_json::to_value(message)?;
        let stmt = txn
            .prepare("select pgmq.send(queue_name => $1, msg => $2)")
            .await?;
        let row = txn.query_one(&stmt, &[queue, &value]).await?;
        Ok(row.get(0))
    }

    pub async fn dequeue<T>(
        &self,
        queue: &String,
        timeout: i32,
        count: i32,
        executor: &impl MessageQueueExecutor<T>,
    ) -> Result<Vec<T>, Error>
    where
        T: Send + Sync,
    {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        match self
            .dequeue_txn(&txn, queue, timeout, count, executor)
            .await
        {
            Ok(result) => {
                txn.commit().await?;
                Ok(result)
            }
            Err(e) => {
                txn.rollback().await?;
                Err(e)
            }
        }
    }

    async fn dequeue_txn<'a, T>(
        &self,
        txn: &'a Transaction<'a>,
        queue: &String,
        timeout: i32,
        count: i32,
        executor: &impl MessageQueueExecutor<T>,
    ) -> Result<Vec<T>, Error>
    where
        T: Send + Sync,
    {
        let stmt = txn
            .prepare_cached("select * from pgmq.read($1, $2, $3)")
            .await?;
        let result = txn.query(&stmt, &[queue, &timeout, &count]).await?;
        let mut messages: Vec<Message> = result.iter().map(|m| m.into()).collect();
        let result = executor.execute(self, txn, queue, &mut messages).await?;
        Ok(result)
    }

    pub async fn retry(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<Message>, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, false).await?;
        let mut query = "update pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" set vt = now() where msg_id = $1 returning *");
        let stmt = txn
            .prepare_cached(query.as_str())
            .await?;
        let result = txn.query(&stmt, &[&id.id]).await?;
        txn.commit().await?;
        Ok(result.first().map(|r| r.into()))
    }

    pub async fn update_message(
        &self,
        id: &WorkflowExecutionId,
        archived: bool,
        executor: &impl MessageQueueExecutor<Message>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let table_name = self.get_table_name_txn(&txn, &id.queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        let message = results.first().map(Message::from);
        if message.is_none() {
            error!(target: "workflow", "couldn't find {id} to update");
            txn.rollback().await?;
            return Err(Error::new("couldn't find message to update"));
        }
        let mut messages = Vec::<Message>::new();
        messages.push(message.unwrap());
        executor.execute(self, &txn, &id.queue, &mut messages).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn update_message_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        id: &WorkflowExecutionId,
        archived: bool,
        executor: &dyn MessageQueueExecutor<Message>,
    ) -> Result<Vec<Message>, Error> {
        let table_name = self.get_table_name_txn(txn, &id.queue, archived).await?;
        let mut query = "select * from pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" where msg_id = $1 for update");
        let stmt = txn.prepare_cached(&query).await?;
        let results = txn.query(&stmt, &[&id.id]).await?;
        let message = results.first().map(Message::from);
        if message.is_none() {
            error!(target: "workflow", "couldn't find {id} (archived: {archived}) to update");
            return Ok(vec![]);
        }
        let mut messages = Vec::<Message>::new();
        messages.push(message.unwrap());
        executor.execute(self, txn, &id.queue, &mut messages).await
    }

    pub async fn update_message_raw_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        queue: &str,
        archived: bool,
        message: &Message,
    ) -> Result<(), Error> {
        let table_name = self.get_table_name_txn(txn, queue, archived).await?;
        let mut query = "update pgmq.\"".to_owned();
        query.push_str(table_name.as_str());
        query.push_str("\" set message = $1, vt = $2 where msg_id = $3");
        let stmt = txn.prepare_cached(&query).await?;
        let value = serde_json::to_value(&message.value)?;
        txn.execute(&stmt, &[&value, &message.visible_timeout, &message.id])
            .await?;
        Ok(())
    }

    async fn get_table_name_txn<'a>(
        &self,
        txn: &'a Transaction<'a>,
        queue: &str,
        archived: bool,
    ) -> Result<String, Error> {
        let stmt = txn
            .prepare_cached(if archived {
                "select pgmq.format_table_name($1, 'a')"
            } else {
                "select pgmq.format_table_name($1, 'q')"
            })
            .await?;
        let queue = queue.to_owned();
        Ok(txn.query_one(&stmt, &[&queue]).await?.get(0))
    }
}
