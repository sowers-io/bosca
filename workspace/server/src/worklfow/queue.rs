use crate::models::workflow::execution_plan::{WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJob, WorkflowJobId};
use crate::redis::RedisClient;
use crate::worklfow::transaction::{Transaction, TransactionOp};
use async_graphql::Error;
use chrono::Utc;
use deadpool_postgres::{GenericClient, Pool};
use log::{debug, error, info, warn};
use redis::{AsyncCommands, Script};
use serde_json::{from_value, json, Value};
use std::str::from_utf8;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct JobQueues {
    pool: Arc<Pool>,
    redis: RedisClient,
}

const QUEUE_PLAN_PREFIX: &str = "queue::plan";
const QUEUE_JOB_PREFIX: &str = "queue::job";

impl JobQueues {
    pub fn new(pool: Arc<Pool>, redis: RedisClient) -> Self {
        Self { pool, redis }
    }

    pub fn queue_plan_key(queue: &str, id: &Uuid) -> String {
        format!("{}::{}::{}", QUEUE_PLAN_PREFIX, queue, id)
    }

    pub fn queue_job_key(queue: &str, id: &Uuid, index: i32) -> String {
        format!("{}::{}::{}::{}", QUEUE_JOB_PREFIX, queue, id, index)
    }

    pub fn queue_key(queue: &str) -> String {
        format!("queue::pending::{}", queue)
    }

    pub fn running_queue_key(queue: &str) -> String {
        format!("queue::running::{}", queue)
    }

    async fn incr(&self, key: &str) -> Result<(), Error> {
        let conn = self.redis.get().await?;
        let mut conn = conn.get_connection().await?;
        let _: i64 = conn.incr(&key, 1).await?;
        Ok(())
    }

    pub async fn get_plan(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select configuration from workflow_plans where id = $1")
            .await?;
        let result = connection.query(&stmt, &[&id.id]).await?;
        let result = result.first();
        if result.is_none() {
            return Err(Error::new("plan not found"));
        }
        let configuration: Value = result.unwrap().get("configuration");
        Ok(Some(from_value::<WorkflowExecutionPlan>(configuration)?))
    }

    pub async fn get_plan_by_job(
        &self,
        id: &WorkflowJobId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let id = WorkflowExecutionId {
            id: id.id.clone(),
            queue: id.queue.clone(),
        };
        self.get_plan(&id).await
    }

    async fn get_plan_and_lock(
        &self,
        transaction: &deadpool_postgres::Transaction<'_>,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let stmt = transaction
            .prepare(
                "select configuration from workflow_plans where id = $1 for update",
            )
            .await?;
        let result = transaction.query(&stmt, &[&id.id]).await?;
        let result = result.first();
        if result.is_none() {
            return Err(Error::new("plan not found"));
        }
        let configuration: Value = result.unwrap().get("configuration");
        Ok(Some(from_value::<WorkflowExecutionPlan>(configuration)?))
    }

    async fn get_plan_and_lock_by_job(
        &self,
        transaction: &deadpool_postgres::Transaction<'_>,
        id: &WorkflowJobId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let id = WorkflowExecutionId {
            id: id.id.clone(),
            queue: id.queue.clone(),
        };
        self.get_plan_and_lock(transaction, &id).await
    }

    async fn set_plan(
        &self,
        transaction: &deadpool_postgres::Transaction<'_>,
        plan: &WorkflowExecutionPlan,
    ) -> Result<(), Error> {
        let stmt = transaction
            .prepare(
                "insert into workflow_plans (id, configuration) values ($1, $2) on conflict (id) do update set configuration = $2, modified = now()",
            )
            .await?;
        let value = json!(plan);
        transaction
            .execute(&stmt, &[&plan.id.id, &value])
            .await?;
        Ok(())
    }

    pub async fn get_all_plans(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        let mut plans = Vec::new();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select configuration from workflow_plans where configuration->>'id'->>'queue' = $1 offset $2 limit $3 order by created desc").await?;
        let result = connection.query(&stmt, &[&queue, &offset, &limit]).await?;
        for row in result {
            let configuration: Value = row.get("configuration");
            plans.push(from_value(configuration)?);
        }
        Ok(plans)
    }

    pub async fn check_for_expiration(&self, time: i64) -> Result<(), Error> {
        let pooled_connection = self.redis.get().await?;
        let mut connection = pooled_connection.get_connection().await?;
        let script = Script::new(
            r"
            local pending_queue = tostring(KEYS[1])
            local running_queue = tostring(KEYS[2])
            local current_timestamp = tonumber(ARGV[1])
            local expired_items = redis.call('ZRANGEBYSCORE', running_queue, 0, current_timestamp)
            if #expired_items > 0 then
                for i, item in ipairs(expired_items) do
                    redis.call('RPUSH', pending_queue, item)
                    redis.call('ZREM', running_queue, item)
                    redis.call('INCR', 'queue::expired::count')
                end
            end
            return #expired_items
        ",
        );

        let queues: Vec<String> = connection.keys("queue::running::*").await?;
        for queue_parts in queues {
            let queue = queue_parts.split("::").last().unwrap();
            let result: i32 = script
                .key(JobQueues::queue_key(&queue))
                .key(JobQueues::running_queue_key(&queue))
                .arg(time)
                .invoke_async(&mut connection)
                .await?;
            if result > 0 {
                error!("found expired jobs: {}", result);
            }
        }

        Ok(())
    }

    pub async fn enqueue(
        &self,
        plan: &WorkflowExecutionPlan,
    ) -> Result<WorkflowExecutionId, Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;
        let mut transaction = Transaction::new();
        transaction.add_op(TransactionOp::QueuePlan(plan.id.clone()));
        transaction.execute(&self.redis).await?;
        self.incr("queue::enqueued::count").await?;
        info!("enqueued plan: {}", plan.id);
        Ok(plan.id.clone())
    }

    // pub async fn plan_exists(&self, id: &WorkflowExecutionId) -> Result<bool, Error> {
    //     let connection = self.pool.get().await?;
    //     let stmt = connection
    //         .prepare("select count(*) from workflow_plans where queue = $1 and id = $2")
    //         .await?;
    //     let result = connection.query_one(&stmt, &[&id.queue, &id.id]).await?;
    //     let count: i64 = result.get("count");
    //     Ok(count > 0)
    // }

    // pub async fn job_exists(&self, id: &WorkflowJobId) -> Result<bool, Error> {
    //     let connection = self.pool.get().await?;
    //     let stmt = connection
    //         .prepare(
    //             "select count(*) from workflow_jobs where queue = $1 and id = $2 and index = $3",
    //         )
    //         .await?;
    //     let result = connection
    //         .query_one(&stmt, &[&id.queue, &id.id, &id.index])
    //         .await?;
    //     let count: i64 = result.get("count");
    //     Ok(count > 0)
    // }

    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowJobId,
        plans: &[WorkflowExecutionPlan],
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        info!(target: "workflow", "enqueuing job children: {}", job_id);
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;

        let Some(mut plan) = self.get_plan_and_lock_by_job(&transaction, &job_id).await? else {
            return Err(Error::new("can't enqueue child workflows, missing job"));
        };
        let job = plan.jobs.get_mut(job_id.index as usize).unwrap();
        if job.complete {
            transaction.rollback().await?;
            return Err(Error::new("job is already complete"));
        }
        let mut ids = Vec::new();
        let mut queue_txn = Transaction::new();
        let mut plans = plans.iter().cloned().collect::<Vec<_>>();
        for plan in plans.iter_mut() {
            plan.parent = Some(job.id.clone());
            job.children.insert(plan.id.clone());
            ids.push(plan.id.clone());
            queue_txn.add_op(TransactionOp::QueuePlan(plan.id.clone()));
            self.set_plan(&transaction, &plan).await?;
            self.incr("queue::enqueued::child::count").await?;
            info!("enqueued plan: {}", plan.id);
        }
        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;
        queue_txn.execute(&self.redis).await?;
        Ok(ids)
    }

    async fn enqueue_execution_job(
        &self,
        id: &WorkflowExecutionId,
        job_index: i32,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;

        let Some(mut plan) = self.get_plan_and_lock(&transaction, &id).await? else {
            return Err(Error::new("can't enqueue job, missing plan"));
        };

        if plan.finished.is_some() {
            transaction.rollback().await?;
            return Err(Error::new("plan is already complete"));
        }

        if plan.complete.contains(&job_index) {
            transaction.rollback().await?;
            return Err(Error::new("job is already complete"));
        }

        if plan.running.contains(&job_index) {
            transaction.rollback().await?;
            return Err(Error::new("job is already running"));
        }

        if !plan.current_execution_group.contains(&job_index)
            && (plan.next.is_none() || !plan.next.as_ref().is_some_and(|id| *id == job_index))
        {
            transaction.rollback().await?;
            error!(target: "workflow", "not enqueuing job, it's not marked as a current execution group or next job: {} {}", id, job_index);
            return Ok(());
        }

        plan.next = None;
        plan.running.insert(job_index);
        plan.pending.remove(&job_index);

        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;

        let job = plan.jobs.get_mut(job_index as usize).unwrap();
        let mut transaction = Transaction::new();
        transaction.add_op(TransactionOp::QueueJob(job.id.clone()));
        transaction.execute(&self.redis).await?;
        self.incr("queue::enqueued::job::count").await?;
        debug!("enqueued plan job: {} {}", plan.id, job.id);
        Ok(())
    }

    async fn dequeue_from_redis(
        &self,
        queue: &str,
    ) -> Result<Option<(WorkflowExecutionPlan, i32)>, Error> {
        let pooled_connection = self.redis.get().await?;
        let mut connection = pooled_connection.get_connection().await?;
        let script = Script::new(
            r"
                local job_queue     = tostring(KEYS[1])
                local running_queue = tostring(KEYS[2])

                local now   = tonumber(ARGV[1]) -- Current timestamp
                local delay = tonumber(ARGV[2]) -- Expiration delay

                local item = redis.call('LPOP', job_queue)
                if item then
                    local expire_time = now + delay
                    redis.call('ZADD', running_queue, expire_time, item)
                    redis.call('INCR', 'queue::dequeued::count')
                    return tostring(item)
                else
                    return nil -- Nothing to pop
                end
            ",
        );
        let result: Vec<u8> = script
            .key(JobQueues::queue_key(queue))
            .key(JobQueues::running_queue_key(queue))
            .arg(Utc::now().timestamp())
            .arg(1800)
            .invoke_async(&mut connection)
            .await?;
        if result.len() == 0 {
            return Ok(None);
        }
        let id = from_utf8(&result)?;
        if id.starts_with(QUEUE_PLAN_PREFIX) {
            let id_parts = id.get(QUEUE_PLAN_PREFIX.len() + 2..).unwrap();
            let mut id_parts = id_parts.split("::");
            let queue = id_parts.next().unwrap();
            let id = Uuid::parse_str(id_parts.next().unwrap())?;
            let id = WorkflowExecutionId {
                id,
                queue: queue.to_owned(),
            };
            let Some(plan) = self.get_plan(&id).await? else {
                return Err(Error::new("can't dequeue job, missing plan"));
            };
            Ok(Some((plan, -1)))
        } else if id.starts_with(QUEUE_JOB_PREFIX) {
            let id_parts = id.get(QUEUE_JOB_PREFIX.len() + 2..).unwrap();
            let mut id_parts = id_parts.split("::");
            let queue = id_parts.next().unwrap();
            let id = Uuid::parse_str(id_parts.next().unwrap())?;
            let index = i32::from_str_radix(id_parts.next().unwrap(), 10)?;
            let id = WorkflowJobId {
                id,
                queue: queue.to_owned(),
                index,
            };
            let Some(plan) = self.get_plan_by_job(&id).await? else {
                return Err(Error::new("can't dequeue job, missing plan"));
            };
            Ok(Some((plan, index)))
        } else {
            return Err(Error::new("unknown prefix"));
        }
    }

    async fn prepare_plan(&self, plan: &mut WorkflowExecutionPlan) -> Result<(), Error> {
        let mut update = false;
        if let Some(id) = &plan.next {
            let job_id = WorkflowJobId {
                id: plan.id.id.clone(),
                queue: plan.id.queue.clone(),
                index: id.clone(),
            };
            warn!(target: "workflow", "plan already has a next: {:?}", job_id);
        } else {
            if plan.current_execution_group.is_empty() && plan.running.is_empty() {
                info!("updating plan current job: {}", plan.id);
                plan.current_execution_group = plan
                    .jobs
                    .iter()
                    .filter(|job| job.workflow_activity.execution_group == 1)
                    .map(|job| job.id.index)
                    .collect();
                update = true;
            }
            if !plan.current_execution_group.is_empty() {
                info!(
                        "removing job from current list and queueing as next: {}",
                        plan.id
                    );
                let next = plan.current_execution_group.remove(0);
                plan.next = Some(next.clone());
                update = true;
            }
            if plan.next.is_none() {
                warn!(target: "workflow", "plan is missing next, not returning: {}", plan.id);
            }
        }
        if update {
            let mut connection = self.pool.get().await?;
            let transaction = connection.transaction().await?;
            self.set_plan(&transaction, &plan).await?;
            transaction.commit().await?;
        }
        Ok(())
    }

    pub async fn dequeue(&self, queue: &str) -> Result<Option<WorkflowJob>, Error> {
        loop {
            let Some((mut plan, mut job_index)) = self.dequeue_from_redis(queue).await? else {
                return Ok(None);
            };

            while plan.finished.is_some() || plan.complete.contains(&job_index) {
                let mut transaction = Transaction::new();
                if plan.finished.is_some() {
                    warn!(target: "workflow", "plan is already complete: {}", plan.id);
                    transaction.add_op(TransactionOp::RemovePlanRunning(plan.id.clone()));
                }
                if job_index != -1 {
                    let id = WorkflowJobId {
                        id: plan.id.id.clone(),
                        queue: plan.id.queue.clone(),
                        index: job_index,
                    };
                    warn!(target: "workflow", "job is already complete: {}", id);
                    transaction.add_op(TransactionOp::RemoveJobRunning(id));
                }
                transaction.execute(&self.redis).await?;
                let Some((plan2, job_index2)) = self.dequeue_from_redis(queue).await? else {
                    return Ok(None);
                };
                plan = plan2;
                job_index = job_index2;
            }

            if job_index != -1 {
                return Ok(Some(plan.jobs.get_mut(job_index as usize).unwrap().clone()))
            } else {
                self.prepare_plan(&mut plan).await?;
                if let Some(next) = plan.next {
                    self.enqueue_execution_job(&plan.id, next).await?;
                }
                continue;
            }
        }
    }

    pub async fn set_execution_plan_context(
        &self,
        plan_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock(&transaction, plan_id).await? else {
            return Err(Error::new("can't set plan context, missing plan"));
        };
        plan.context = context.clone();
        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;
        self.incr("queue::context::set::count").await?;
        Ok(())
    }

    pub async fn set_execution_job_context(
        &self,
        job_id: &WorkflowJobId,
        context: &Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&transaction, job_id).await? else {
            return Err(Error::new("can't set job context, missing plan"));
        };
        let job = plan.jobs.get_mut(job_id.index as usize).unwrap();
        job.context = context.clone();
        if let Some(plan_job) = plan.jobs.get_mut(job_id.index as usize) {
            plan_job.context = context.clone();
        }
        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;
        self.incr("queue::context::job::set::count").await?;
        Ok(())
    }

    pub async fn set_execution_plan_job_failed(
        &self,
        job_id: &WorkflowJobId,
        error: &str,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&transaction, job_id).await? else {
            return Err(Error::new("can't set job context, missing plan"));
        };
        plan.failed.insert(job_id.index);
        plan.running.remove(&job_id.index);
        if !plan.current_execution_group.contains(&job_id.index) {
            plan.current_execution_group.push(job_id.index);
        }
        let job = plan.jobs.get_mut(job_id.index as usize).unwrap();
        job.error = Some(error.to_owned());
        self.set_plan(&transaction, &plan).await?;
        transaction.commit().await?;
        self.incr("queue::job::failed").await?;
        Ok(())
    }

    pub async fn set_execution_plan_job_checkin(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let pooled_connection = self.redis.get().await?;
        let mut connection = pooled_connection.get_connection().await?;
        let script = Script::new(
            r"
            local running_queue = tostring(KEYS[1])
            local now           = tonumber(ARGV[1]) -- Current timestamp
            local delay         = tonumber(ARGV[2]) -- Expiration delay
            local expire_time   = now + delay
            redis.call('ZADD', running_queue, expire_time, item)
            redis.call('INCR', 'queue::job::checkin::count')
            return 0
        ",
        );
        let result: i32 = script
            .key(JobQueues::running_queue_key(&job_id.queue))
            .arg(Utc::now().timestamp())
            .arg(1800)
            .invoke_async(&mut connection)
            .await?;
        if result != 0 {
            return Err(Error::new("invalid result"));
        }
        Ok(())
    }

    pub async fn set_execution_plan_job_complete(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;

        let Some(mut plan) = self.get_plan_and_lock_by_job(&transaction, job_id).await? else {
            return Err(Error::new("can't mark execution complete, missing job"));
        };
        let mut redis_txn = Transaction::new();

        match self
            .set_execution_job_complete_recursive(
                &transaction,
                &mut redis_txn,
                &mut plan,
                job_id.index,
            )
            .await
        {
            Ok(_) => {
                transaction.commit().await?;
                redis_txn.execute(&self.redis).await?;
                self.incr("queue::job::complete").await?;
                Ok(())
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }

    async fn set_execution_job_complete_recursive(
        &self,
        db_txn: &deadpool_postgres::Transaction<'_>,
        redis_tx: &mut Transaction,
        plan: &mut WorkflowExecutionPlan,
        job_index: i32,
    ) -> Result<(), Error> {
        let job = plan.jobs.get_mut(job_index as usize).unwrap();
        job.error = None;
        job.complete = job.children.len() == job.completed_children.len();
        if job.complete {
            job.finished = Some(Utc::now());
        }
        redis_tx.add_op(TransactionOp::RemoveJobRunning(job.id.clone()));

        plan.failed.remove(&job.id.index);
        plan.running.remove(&job.id.index);
        plan.complete.insert(job.id.index);

        let mut dirty_plan = true;

        if plan.running.is_empty() && plan.current_execution_group.is_empty() {
            let next_execution_group = job.workflow_activity.execution_group + 1;
            info!(target: "workflow", "no running jobs, checking for next group: {} - execution group: {}", plan.id, next_execution_group);
            let new_current: Vec<i32> = plan
                .jobs
                .iter()
                .filter(|job| {
                    !job.complete && job.workflow_activity.execution_group == next_execution_group
                })
                .map(|job| job.id.index)
                .collect();
            if new_current.is_empty() {
                info!(target: "workflow", "plan doesn't have any current jobs, finishing: {}", plan.id);
                plan.finished = Some(Utc::now());
                if let Some(parent_id) = &plan.parent {
                    let Some(mut parent_plan) =
                        self.get_plan_and_lock_by_job(db_txn, parent_id).await?
                    else {
                        return Err(Error::new(
                            "can't mark execution complete, missing parent job",
                        ));
                    };
                    let parent_job = parent_plan.jobs.get_mut(parent_id.index as usize).unwrap();
                    parent_job.completed_children.insert(plan.id.clone());
                    if parent_job.children.len() == parent_job.completed_children.len() {
                        self.set_plan(db_txn, &plan).await?;
                        dirty_plan = false;
                        self.incr("queue::job::complete::recursive").await?;
                        Box::pin(self.set_execution_job_complete_recursive(
                            db_txn,
                            redis_tx,
                            &mut parent_plan,
                            parent_id.index,
                        ))
                        .await?;
                    } else {
                        self.set_plan(db_txn, &parent_plan).await?;
                    }
                }
                redis_tx.add_op(TransactionOp::RemovePlanRunning(plan.id.clone()));
            } else {
                info!(target: "workflow", "marking plan as ready for processing on the next execution group: {}", plan.id);
                plan.current_execution_group = new_current;
                redis_tx.add_op(TransactionOp::QueuePlan(plan.id.clone()));
            }
        } else {
            redis_tx.add_op(TransactionOp::QueuePlan(plan.id.clone()));
        }

        if dirty_plan {
            self.set_plan(db_txn, &plan).await?;
        }

        Ok(())
    }
}
