use crate::models::workflow::execution_plan::{WorkflowExecutionId, WorkflowJobId};
use crate::redis::RedisClient;
use crate::workflow::queue::JobQueues;
use async_graphql::Error;
use chrono::Utc;
use log::error;
use redis::Script;
use uuid::Uuid;

pub struct RedisTransaction {
    ops: Vec<RedisTransactionOp>,
}

impl RedisTransaction {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn add_op(&mut self, op: RedisTransactionOp) {
        self.ops.push(op);
    }

    pub async fn execute(&self, redis: &RedisClient) -> Result<(), Error> {
        let mut script = "".to_string();
        let mut key_ix = 0;
        let mut arg_ix = 0;
        for op in &self.ops {
            match op {
                RedisTransactionOp::QueueJob(_) => {
                    let rpush = format!(
                        "redis.call('RPUSH', tostring(KEYS[{}]), tostring(KEYS[{}]))\n",
                        key_ix + 1,
                        key_ix + 2
                    );
                    key_ix += 2;
                    script.push_str(&rpush);
                }
                RedisTransactionOp::CancelQueueJob(_)
                | RedisTransactionOp::RemovePlanRunning(_)
                | RedisTransactionOp::RemoveJobRunning(_) => {
                    let zrem = format!(
                        "redis.call('ZREM', tostring(KEYS[{}]), tostring(KEYS[{}]))\n",
                        key_ix + 1,
                        key_ix + 2
                    );
                    key_ix += 2;
                    script.push_str(&zrem);
                }
                RedisTransactionOp::PlanCheckin(_)
                | RedisTransactionOp::JobCheckin(_)
                | RedisTransactionOp::QueueJobLater(_, _) => {
                    let zadd_incr = format!("redis.call('ZADD', tostring(KEYS[{}]), tonumber(ARGV[{}]) + tonumber(ARGV[{}]), tostring(KEYS[{}]))\nredis.call('INCR', 'queue::job::checkin::count')\n", key_ix + 1, arg_ix + 1, arg_ix + 2, key_ix + 2);
                    key_ix += 2;
                    arg_ix += 2;
                    script.push_str(&zadd_incr);
                }
                RedisTransactionOp::AddMetadataRunning(_)
                | RedisTransactionOp::AddCollectionRunning(_) => {
                    let incrby = format!(
                        "redis.call('HINCRBY', 'running::metadata', tostring(KEYS[{}]), 1)\n",
                        key_ix + 1
                    );
                    script.push_str(&incrby);
                    key_ix += 1;
                }
                RedisTransactionOp::RemoveMetadataRunning(_)
                | RedisTransactionOp::RemoveCollectionRunning(_) => {
                    let incrby = format!("local c = redis.call('HINCRBY', 'running::metadata', tostring(KEYS[{}]), -1)\nif c <= 0 then\nredis.call('HDEL', 'running::metadata', tostring(KEYS[{}]))\nend\n", key_ix + 1, key_ix + 2);
                    script.push_str(&incrby);
                    key_ix += 2;
                }
            }
        }
        script.push_str("return 1\n");
        let script = Script::new(&script);
        let mut invocation = script.prepare_invoke();
        for op in &self.ops {
            match op {
                RedisTransactionOp::QueueJob(op) => {
                    let queue_key = JobQueues::pending_job_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation.key(&queue_key).key(&key);
                }
                RedisTransactionOp::QueueJobLater(op, timeout) => {
                    // putting in running queue so that when the timeout checker will find this
                    // and re-run it later.  TODO: maybe do this differently
                    let queue_key = JobQueues::running_job_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation
                        .key(&queue_key)
                        .key(&key)
                        .arg(Utc::now().timestamp())
                        .arg(timeout);
                }
                RedisTransactionOp::CancelQueueJob(op) => {
                    let queue_key = JobQueues::pending_job_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation.key(&queue_key).key(&key);
                }
                RedisTransactionOp::RemovePlanRunning(op) => {
                    let queue_key = JobQueues::running_plan_queue_key(&op.queue);
                    let key = JobQueues::queue_plan_key(&op.queue, &op.id);
                    invocation.key(&queue_key).key(&key);
                }
                RedisTransactionOp::RemoveJobRunning(op) => {
                    let queue_key = JobQueues::running_job_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation.key(&queue_key).key(&key);
                }
                RedisTransactionOp::PlanCheckin(op) => {
                    let queue_key = JobQueues::running_plan_queue_key(&op.queue);
                    let key = JobQueues::queue_plan_key(&op.queue, &op.id);
                    invocation
                        .key(&queue_key)
                        .key(&key)
                        .arg(Utc::now().timestamp())
                        .arg(1800);
                }
                RedisTransactionOp::JobCheckin(op) => {
                    let queue_key = JobQueues::running_job_queue_key(&op.queue);
                    let key = JobQueues::queue_job_key(&op.queue, &op.id, op.index);
                    invocation
                        .key(&queue_key)
                        .key(&key)
                        .arg(Utc::now().timestamp())
                        .arg(1800);
                }
                RedisTransactionOp::AddMetadataRunning(op) => {
                    let key = op.to_string();
                    invocation
                        .key(&key);
                }
                RedisTransactionOp::AddCollectionRunning(op) => {
                    let key = op.to_string();
                    invocation
                        .key(&key);
                }
                RedisTransactionOp::RemoveMetadataRunning(op) => {
                    let key = op.to_string();
                    invocation
                        .key(&key)
                        .key(&key);
                }
                RedisTransactionOp::RemoveCollectionRunning(op) => {
                    let key = op.to_string();
                    invocation
                        .key(&key)
                        .key(&key);
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
                error!(target: "workflow", "{e:?}");
                Err(e.into())
            }
        }
    }
}

pub enum RedisTransactionOp {
    PlanCheckin(WorkflowExecutionId),
    JobCheckin(WorkflowJobId),
    QueueJob(WorkflowJobId),
    QueueJobLater(WorkflowJobId, i64),
    CancelQueueJob(WorkflowJobId),
    RemovePlanRunning(WorkflowExecutionId),
    RemoveJobRunning(WorkflowJobId),
    AddMetadataRunning(Uuid),
    AddCollectionRunning(Uuid),
    RemoveMetadataRunning(Uuid),
    RemoveCollectionRunning(Uuid),
}
