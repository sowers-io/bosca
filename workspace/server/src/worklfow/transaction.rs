use async_graphql::Error;
use log::error;
use redis::Script;
use crate::models::workflow::execution_plan::{WorkflowExecutionId, WorkflowJobId};
use crate::redis::RedisClient;
use crate::worklfow::queue::JobQueues;

pub struct Transaction {
    ops: Vec<TransactionOp>,
}

impl Transaction {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn add_op(&mut self, op: TransactionOp) {
        self.ops.push(op);
    }

    pub async fn execute(&self, redis: &RedisClient) -> Result<(), Error> {
        let mut script = "".to_string();
        let mut key_ix = 0;
        for op in &self.ops {
            match op {
                TransactionOp::QueuePlan(_) | TransactionOp::QueueJob(_) => {
                    let rpush = format!("redis.call('RPUSH', tostring(KEYS[{}]), tostring(KEYS[{}]))\n", key_ix + 1, key_ix + 2);
                    key_ix += 2;
                    script.push_str(&rpush);
                }
                TransactionOp::RemovePlanRunning(_) | TransactionOp::RemoveJobRunning(_) => {
                    let zrem = format!("redis.call('ZREM', tostring(KEYS[{}]), tostring(KEYS[{}]))\n", key_ix + 1, key_ix + 2);
                    key_ix += 2;
                    script.push_str(&zrem);
                }
            }
        }
        script.push_str("return 1\n");
        let script = Script::new(&script);
        let mut invocation = script.prepare_invoke();
        for op in &self.ops {
            match op {
                TransactionOp::QueuePlan(op) => {
                    let queue_key = JobQueues::queue_key(&op.queue);
                    let key = JobQueues::queue_plan_key(&op.queue, &op.id);
                    invocation.key(&queue_key).key(&key);
                }
                TransactionOp::QueueJob(op) => {
                    let queue_key = JobQueues::queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation.key(&queue_key).key(&key);
                }
                TransactionOp::RemovePlanRunning(op) => {
                    let queue_key = JobQueues::running_queue_key(&op.queue);
                    let key = JobQueues::queue_plan_key(&op.queue, &op.id);
                    invocation.key(&queue_key).key(&key);
                }
                TransactionOp::RemoveJobRunning(op) => {
                    let queue_key = JobQueues::running_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation.key(&queue_key).key(&key);
                }
            }
        }
        let connection = redis.get().await?;
        let mut conn = connection.get_connection().await?;
        match invocation.invoke_async(&mut conn).await {
            Ok(result) => {
                let result: i32 = result;
                if result != 1 {
                    return Err(Error::new("script failed"));
                }
                Ok(())
            }
            Err(e) => {
                error!(target: "workflow", "{:?}", e);
                Err(e.into())
            }
        }
    }
}

pub enum TransactionOp {
    QueuePlan(WorkflowExecutionId),
    QueueJob(WorkflowJobId),
    RemovePlanRunning(WorkflowExecutionId),
    RemoveJobRunning(WorkflowJobId),
}