use crate::datastores::notifier::Notifier;
use crate::models::workflow::execution_plan::{
    WorkflowExecutePlanState, WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJob,
    WorkflowJobId,
};
use crate::redis::RedisClient;
use crate::worklfow::transaction::RedisTransaction;
use async_graphql::Error;
use chrono::Utc;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::{error, info};
use redis::{AsyncCommands, Script};
use serde_json::{from_value, json, Value};
use std::collections::HashSet;
use std::str::from_utf8;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct JobQueues {
    pool: Arc<Pool>,
    redis: RedisClient,
    notifier: Arc<Notifier>,
}

const QUEUE_PLAN_PREFIX: &str = "queue::plan";
const QUEUE_JOB_PREFIX: &str = "queue::job";

impl JobQueues {
    pub fn new(pool: Arc<Pool>, redis: RedisClient, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            redis,
            notifier,
        }
    }

    pub fn queue_plan_key(queue: &str, id: &Uuid) -> String {
        format!("{}::{}::{}", QUEUE_PLAN_PREFIX, queue, id)
    }

    pub fn queue_job_key(queue: &str, id: &Uuid, index: i32) -> String {
        format!("{}::{}::{}::{}", QUEUE_JOB_PREFIX, queue, id, index)
    }

    pub fn pending_plan_queue_key(queue: &str) -> String {
        format!("queue::plan::pending::{}", queue)
    }

    pub fn pending_job_queue_key(queue: &str) -> String {
        format!("queue::job::pending::{}", queue)
    }

    pub fn running_plan_queue_key(queue: &str) -> String {
        format!("queue::plan::running::{}", queue)
    }

    pub fn running_job_queue_key(queue: &str) -> String {
        format!("queue::job::running::{}", queue)
    }

    async fn incr(&self, key: &str) -> Result<(), Error> {
        let conn = self.redis.get().await?;
        let mut conn = conn.get_connection().await?;
        let _: i64 = conn.incr(key, 1).await?;
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
            id: id.id,
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
            .prepare("select configuration from workflow_plans where id = $1 for update")
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
        transaction: &Transaction<'_>,
        id: &WorkflowJobId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let id = WorkflowExecutionId {
            id: id.id,
            queue: id.queue.clone(),
        };
        self.get_plan_and_lock(transaction, &id).await
    }

    async fn set_plan(
        &self,
        transaction: &Transaction<'_>,
        plan: &WorkflowExecutionPlan,
        register: bool,
    ) -> Result<(), Error> {
        let stmt = transaction
            .prepare(
                "insert into workflow_plans (id, configuration) values ($1, $2) on conflict (id) do update set configuration = $2, modified = now()",
            )
            .await?;
        let value = json!(plan);
        transaction.execute(&stmt, &[&plan.id.id, &value]).await?;
        if register {
            if let Some(metadata_id) = &plan.metadata_id {
                let stmt = transaction
                    .prepare_cached(
                        "insert into metadata_workflow_plans (id, plan_id, queue) values ($1, $2, $3) on conflict do nothing",
                    )
                    .await?;
                let plan_id = &plan.id;
                transaction
                    .execute(&stmt, &[metadata_id, &plan_id.id, &plan_id.queue])
                    .await?;
            }
            if let Some(collection_id) = &plan.collection_id {
                let stmt = transaction
                    .prepare_cached(
                        "insert into collection_workflow_plans (id, plan_id, queue) values ($1, $2, $3) on conflict do nothing",
                    )
                    .await?;
                let plan_id = &plan.id;
                transaction
                    .execute(&stmt, &[collection_id, &plan_id.id, &plan_id.queue])
                    .await?;
            }
        }
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
                .key(JobQueues::pending_job_queue_key(queue))
                .key(JobQueues::running_job_queue_key(queue))
                .arg(time)
                .invoke_async(&mut connection)
                .await?;
            if result > 0 {
                error!("found expired jobs: {}", result);
            }
        }

        Ok(())
    }

    pub async fn enqueue_plan(
        &self,
        plan: &mut WorkflowExecutionPlan,
    ) -> Result<WorkflowExecutionId, Error> {
        info!(target: "workflow", "enqueuing plan: {}", plan.id);
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;
        let mut redis_txn = RedisTransaction::new();
        let resolver = WorkflowExecutionPlanResolver {
            queue: self.clone(),
        };
        let state = plan.update(&db_txn, &mut redis_txn, &resolver, 1).await?;
        if state == WorkflowExecutePlanState::Complete {
            return Err(Error::new("can't enqueue plan, it's already complete"));
        }
        if state == WorkflowExecutePlanState::Error {
            return Err(Error::new("can't enqueue plan, it has a state error"));
        }
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        self.incr("queue::enqueued::count").await?;
        info!("enqueued plan: {}", plan.id);
        if let Some(id) = &plan.collection_id {
            self.notifier.collection_changed(id).await?;
        }
        if let Some(id) = &plan.metadata_id {
            self.notifier.metadata_changed(id).await?;
        }
        Ok(plan.id.clone())
    }

    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowJobId,
        plans: &[WorkflowExecutionPlan],
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        info!(target: "workflow", "enqueuing job children: {}", job_id);
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;

        let Some(mut plan) = self.get_plan_and_lock_by_job(&db_txn, job_id).await? else {
            return Err(Error::new("can't enqueue child workflows, missing job"));
        };
        let job = plan.jobs.get_mut(job_id.index as usize).unwrap();
        if job.complete {
            db_txn.rollback().await?;
            return Err(Error::new("job is already complete"));
        }
        let resolver = WorkflowExecutionPlanResolver {
            queue: self.clone(),
        };
        let mut ids = Vec::new();
        let mut redis_txn = RedisTransaction::new();
        let mut plans = plans.to_vec();
        let mut collection_ids = HashSet::new();
        let mut metadata_ids = HashSet::new();
        for plan in plans.iter_mut() {
            plan.parent = Some(job.id.clone());
            let state = plan.update(&db_txn, &mut redis_txn, &resolver, 1).await?;
            if state == WorkflowExecutePlanState::Complete {
                db_txn.rollback().await?;
                return Err(Error::new("can't enqueue plan, it's already complete"));
            }
            if state == WorkflowExecutePlanState::Error {
                db_txn.rollback().await?;
                return Err(Error::new("can't enqueue plan, it has a state error"));
            }
            job.children.insert(plan.id.clone());
            ids.push(plan.id.clone());
            if let Some(id) = &plan.collection_id {
                collection_ids.insert(*id);
            }
            if let Some(id) = &plan.metadata_id {
                metadata_ids.insert(*id);
            }
            self.incr("queue::enqueued::child::count").await?;
            info!("enqueued plan: {}", plan.id);
        }
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        for id in collection_ids {
            self.notifier.collection_changed(&id).await?;
        }
        for id in metadata_ids {
            self.notifier.metadata_changed(&id).await?;
        }
        Ok(ids)
    }

    fn new_dequeue_script(&self) -> Script {
        Script::new(
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
        )
    }

    async fn dequeue_from_redis(
        &self,
        pending_key: &str,
        running_key: &str,
    ) -> Result<Option<String>, Error> {
        let pooled_connection = self.redis.get().await?;
        let mut connection = pooled_connection.get_connection().await?;
        let script = self.new_dequeue_script();
        let result: Vec<u8> = script
            .key(pending_key)
            .key(running_key)
            .arg(Utc::now().timestamp())
            .arg(1800)
            .invoke_async(&mut connection)
            .await?;
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(from_utf8(&result)?.to_owned()))
        }
    }

    async fn dequeue_plan_from_redis(
        &self,
        transaction: &Transaction<'_>,
        queue: &str,
    ) -> Result<Option<(WorkflowExecutionPlan, i32)>, Error> {
        let pending_key = JobQueues::pending_plan_queue_key(queue);
        let running_key = JobQueues::running_plan_queue_key(queue);
        if let Some(id) = self.dequeue_from_redis(&pending_key, &running_key).await? {
            let id_parts = id.get(QUEUE_PLAN_PREFIX.len() + 2..).unwrap();
            let mut id_parts = id_parts.split("::");
            let queue = id_parts.next().unwrap();
            let id = Uuid::parse_str(id_parts.next().unwrap())?;
            let id = WorkflowExecutionId {
                id,
                queue: queue.to_owned(),
            };
            let Some(plan) = self.get_plan_and_lock(transaction, &id).await? else {
                return Err(Error::new("can't dequeue job, missing plan"));
            };
            Ok(Some((plan, -1)))
        } else {
            Ok(None)
        }
    }

    async fn dequeue_job_from_redis(
        &self,
        transaction: &Transaction<'_>,
        queue: &str,
    ) -> Result<Option<(WorkflowExecutionPlan, i32)>, Error> {
        let pending_key = JobQueues::pending_job_queue_key(queue);
        let running_key = JobQueues::running_job_queue_key(queue);
        if let Some(id) = self.dequeue_from_redis(&pending_key, &running_key).await? {
            let id_parts = id.get(QUEUE_JOB_PREFIX.len() + 2..).unwrap();
            let mut id_parts = id_parts.split("::");
            let queue = id_parts.next().unwrap();
            let id = Uuid::parse_str(id_parts.next().unwrap())?;
            let index = id_parts.next().unwrap().parse::<i32>()?;
            let id = WorkflowJobId {
                id,
                queue: queue.to_owned(),
                index,
            };
            let Some(plan) = self.get_plan_and_lock_by_job(transaction, &id).await? else {
                return Err(Error::new("can't dequeue job, missing plan"));
            };
            Ok(Some((plan, index)))
        } else {
            Ok(None)
        }
    }

    pub async fn dequeue(&self, queue: &str) -> Result<Option<WorkflowJob>, Error> {
        let mut connection = self.pool.get().await?;
        loop {
            let transaction = connection.transaction().await?;
            let Some((plan, job_index)) = self.dequeue_job_from_redis(&transaction, queue).await?
            else {
                transaction.rollback().await?;
                return Ok(None);
            };
            if job_index != -1 {
                transaction.commit().await?;
                return Ok(Some(plan.jobs.get(job_index as usize).unwrap().clone()));
            }
            transaction.commit().await?;
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
        self.set_plan(&transaction, &plan, false).await?;
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
        self.set_plan(&transaction, &plan, false).await?;
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
        let db_txn = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&db_txn, job_id).await? else {
            return Err(Error::new("can't set job context, missing plan"));
        };
        let mut redis_txn = RedisTransaction::new();
        let resolver = WorkflowExecutionPlanResolver {
            queue: self.clone(),
        };
        plan.set_job_failed(job_id, &db_txn, &mut redis_txn, &resolver, error).await?;
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
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
            local item          = tostring(KEYS[2])
            local now           = tonumber(ARGV[1]) -- Current timestamp
            local delay         = tonumber(ARGV[2]) -- Expiration delay
            local expire_time   = now + delay
            redis.call('ZADD', running_queue, expire_time, item)
            redis.call('INCR', 'queue::job::checkin::count')
            return 0
        ",
        );
        let result: i32 = script
            .key(JobQueues::running_job_queue_key(&job_id.queue))
            .key(JobQueues::queue_job_key(
                &job_id.queue,
                &job_id.id,
                job_id.index,
            ))
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
        let mut redis_txn = RedisTransaction::new();
        let resolver = WorkflowExecutionPlanResolver {
            queue: self.clone(),
        };
        match plan.try_set_job_complete(&transaction, &mut redis_txn, &resolver, job_id).await {
            Ok(result) => {
                transaction.commit().await?;
                redis_txn.execute(&self.redis).await?;
                self.incr("queue::job::complete").await?;
                if result == WorkflowExecutePlanState::Error {
                    return Err(Error::new("plan is in an error state"));
                }
                Ok(())
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }
}

pub struct WorkflowExecutionPlanResolver {
    queue: JobQueues,
}

impl WorkflowExecutionPlanResolver {
    pub async fn get_plan(
        &self,
        txn: &Transaction<'_>,
        job_id: &WorkflowJobId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        self.queue.get_plan_and_lock_by_job(txn, job_id).await
    }

    pub async fn set_plan(
        &self,
        txn: &Transaction<'_>,
        plan: &WorkflowExecutionPlan,
        register: bool,
    ) -> Result<(), Error> {
        self.queue.set_plan(txn, plan, register).await
    }
}
