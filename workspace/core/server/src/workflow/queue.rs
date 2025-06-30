use crate::datastores::notifier::Notifier;
use crate::models::workflow::execution_plan::{
    WorkflowExecutePlanState, WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJob,
    WorkflowJobId,
};
use crate::redis::RedisClient;
use crate::workflow::transaction::RedisTransactionOp::{
    CancelQueueJob, JobCheckin, RemoveJobRunning, RemovePlanRunning,
};
use crate::workflow::transaction::{RedisTransaction, RedisTransactionOp};
use async_graphql::Error;
use chrono::{DateTime, Utc};
use deadpool_postgres::{GenericClient, Transaction};
use log::{debug, error, info, warn};
use redis::{AsyncCommands, Script};
use serde_json::{from_value, json, Value};
use std::collections::HashSet;
use std::str::from_utf8;
use std::sync::Arc;
use uuid::Uuid;
use bosca_database::TracingPool;

#[derive(Clone)]
pub struct JobQueues {
    pool: TracingPool,
    redis: RedisClient,
    notifier: Arc<Notifier>,
}

const QUEUE_PLAN_PREFIX: &str = "queue::plan";
const QUEUE_JOB_PREFIX: &str = "queue::job";

impl JobQueues {
    pub fn new(pool: TracingPool, redis: RedisClient, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            redis,
            notifier,
        }
    }

    pub fn queue_plan_key(queue: &str, id: &Uuid) -> String {
        format!("{QUEUE_PLAN_PREFIX}::{queue}::{id}")
    }

    pub fn queue_job_key(queue: &str, id: &Uuid, index: i32) -> String {
        format!("{QUEUE_JOB_PREFIX}::{queue}::{id}::{index}")
    }

    pub fn pending_job_queue_key(queue: &str) -> String {
        format!("queue::pending::job::{queue}")
    }

    pub fn running_plan_queue_key(queue: &str) -> String {
        format!("queue::running::plan::{queue}")
    }

    pub fn running_job_queue_key(queue: &str) -> String {
        format!("queue::running::job::{queue}")
    }

    async fn incr(&self, key: &str) -> Result<(), Error> {
        let conn = self.redis.get().await?;
        let mut conn = conn.get_connection().await?;
        let _: i64 = conn.incr(key, 1).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_metadata_count(&self, id: &Uuid) -> Result<i64, Error> {
        let redis = self.redis.get().await?;
        let mut conn = redis.get_connection().await?;
        let id = id.to_string();
        if let Some(count) = conn.hget("running::metadata", &id).await? {
            Ok(count)
        } else {
            Ok(0)
        }
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_collection_count(&self, id: &Uuid) -> Result<i64, Error> {
        let redis = self.redis.get().await?;
        let mut conn = redis.get_connection().await?;
        let id = id.to_string();
        if let Some(count) = conn.hget("running::collections", &id).await? {
            Ok(count)
        } else {
            Ok(0)
        }
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_plan(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select configuration from workflow_plans where id = $1")
            .await?;
        let result = connection.query(&stmt, &[&id.id]).await?;
        if let Some(result) = result.first() {
            let configuration: Value = result.get("configuration");
            Ok(Some(from_value::<WorkflowExecutionPlan>(configuration)?))
        } else {
            Err(Error::new("plan not found"))
        }
    }

    #[tracing::instrument(skip(self, id))]
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

    #[tracing::instrument(skip(self, transaction, id))]
    pub async fn get_plan_and_lock(
        &self,
        transaction: &Transaction<'_>,
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

    #[tracing::instrument(skip(self, transaction, id))]
    pub async fn get_plan_and_lock_by_job(
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

    #[tracing::instrument(skip(self, workflow_id, metadata_id, metadata_version, collection_id))]
    pub async fn cancel_workflows(
        &self,
        id: &Option<Uuid>,
        workflow_id: &Option<String>,
        metadata_id: &Option<Uuid>,
        metadata_version: &Option<i32>,
        collection_id: &Option<Uuid>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let workflow_id = workflow_id.to_owned();
        let results = if let Some(collection_id) = collection_id {
            let stmt = connection.prepare("select id, queue from workflow_plans where collection_id = $1 and workflow_id = $2 and finished is null").await?;
            connection
                .query(&stmt, &[collection_id, &workflow_id])
                .await?
        } else if let Some(metadata_id) = metadata_id {
            let stmt = connection.prepare("select id, queue from workflow_plans where metadata_id = $1 and metadata_version = $2 and workflow_id = $3 and finished is null").await?;
            connection
                .query(&stmt, &[metadata_id, metadata_version, &workflow_id])
                .await?
        } else if let Some(workflow_id) = workflow_id {
            let stmt = connection.prepare("select id, queue from workflow_plans where workflow_id = $1 and finished is null").await?;
            connection
                .query(&stmt, &[&workflow_id])
                .await?
        } else if let Some(id) = id {
            let stmt = connection.prepare("select id, queue from workflow_plans where id = $1 and finished is null").await?;
            connection
                .query(&stmt, &[id])
                .await?
        } else {
            return Err(Error::new("invalid request specified"));
        };

        let db_txn = connection.transaction().await?;
        let mut redis_txn = RedisTransaction::new();

        let mut collection_ids = HashSet::new();
        let mut metadata_ids = HashSet::new();

        for row in results {
            let id = row.get("id");
            let queue = row.get("queue");
            let id = WorkflowExecutionId { id, queue };
            let Some(mut plan) = self.get_plan_and_lock(&db_txn, &id).await? else {
                continue;
            };
            if plan.metadata_id.is_some() && plan.metadata_version.is_some() {
                metadata_ids.insert(plan.metadata_id.unwrap());
            }
            if plan.collection_id.is_some() {
                collection_ids.insert(plan.collection_id.unwrap());
            }
            for job in plan.jobs.iter_mut() {
                job.finished = Some(Utc::now());
                redis_txn.add_op(CancelQueueJob(job.id.clone()));
                redis_txn.add_op(RemoveJobRunning(job.id.clone()));
            }
            plan.active.clear();
            plan.finished = Some(Utc::now());
            plan.cancelled = true;
            redis_txn.add_op(RemovePlanRunning(plan.id.clone()));
            if let Some(metadata_id) = plan.metadata_id {
                redis_txn.add_op(RedisTransactionOp::RemoveMetadataRunning(metadata_id));
            }
            if let Some(collection_id) = plan.collection_id {
                redis_txn.add_op(RedisTransactionOp::RemoveCollectionRunning(collection_id));
            }
            self.set_plan(&db_txn, &plan, false).await?;
        }

        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;

        for id in collection_ids {
            self.notifier.collection_changed(&id).await?;
        }
        for id in metadata_ids {
            self.notifier.metadata_changed(&id).await?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, transaction, plan, register))]
    pub async fn set_plan(
        &self,
        transaction: &Transaction<'_>,
        plan: &WorkflowExecutionPlan,
        register: bool,
    ) -> Result<(), Error> {
        let stmt = transaction
            .prepare(
                "insert into workflow_plans (id, queue, workflow_id, metadata_id, metadata_version, collection_id, finished, configuration) values ($1, $2, $3, $4, $5, $6, $7, $8) on conflict (id) do update set finished = $7, configuration = $8, modified = now()",
            )
            .await?;
        let value = json!(plan);
        transaction
            .execute(
                &stmt,
                &[
                    &plan.id.id,
                    &plan.id.queue,
                    &plan.workflow.id,
                    &plan.metadata_id,
                    &plan.metadata_version,
                    &plan.collection_id,
                    &plan.finished,
                    &value,
                ],
            )
            .await?;
        if register {
            if let Some(metadata_id) = &plan.metadata_id {
                let stmt = transaction
                    .prepare_cached(
                        "insert into metadata_workflow_plans (id, plan_id, queue) values ($1, $2, $3) on conflict do nothing",
                    )
                    .await?;
                let plan_id = &plan.id;
                if let Err(e) = transaction
                    .execute(&stmt, &[metadata_id, &plan_id.id, &plan_id.queue])
                    .await
                {
                    error!("failed to register metadata workflow plan: {e}");
                }
            }
            if let Some(collection_id) = &plan.collection_id {
                let stmt = transaction
                    .prepare_cached(
                        "insert into collection_workflow_plans (id, plan_id, queue) values ($1, $2, $3) on conflict do nothing",
                    )
                    .await?;
                let plan_id = &plan.id;
                if let Err(e) = transaction
                    .execute(&stmt, &[collection_id, &plan_id.id, &plan_id.queue])
                    .await
                {
                    error!("failed to register collection workflow plan: {e}");
                }
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, queue, offset, limit))]
    pub async fn get_all_plans(
        &self,
        queue: Option<String>,
        offset: i64,
        limit: i64,
        active: Option<bool>,
        failures: Option<bool>,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        let mut plans = Vec::new();
        let connection = self.pool.get().await?;
        let mut query = "select configuration from workflow_plans".to_string();
        let mut filter = "".to_string();
        let mut ix = 1;
        if let Some(queue) = &queue {
            if !queue.is_empty() {
                if !filter.is_empty() {
                    filter.push_str(" and ");
                }
                filter.push_str("configuration->'id'->>'queue' = $1");
                ix += 1;
            }
        }
        if failures.unwrap_or(false) {
            if !filter.is_empty() {
                filter.push_str(" and ");
            }
            filter.push_str("jsonb_array_length(configuration->'failed') > 0");
        }
        if active.unwrap_or(false) {
            if !filter.is_empty() {
                filter.push_str(" and ");
            }
            filter.push_str("jsonb_array_length(configuration->'active') > 0");
        }
        if !filter.is_empty() {
            query.push_str(&format!(" where {filter}"));
        }
        query.push_str(&format!(" order by created desc offset ${} limit ${}", ix, ix + 1));
        let result = if let Some(queue) = queue {
            if queue.is_empty() {
                let stmt = connection.prepare(&query).await?;
                connection.query(&stmt, &[&offset, &limit]).await?
            } else {
                let stmt = connection.prepare(&query).await?;
                connection.query(&stmt, &[&queue, &offset, &limit]).await?
            }
        } else {
            let stmt = connection.prepare(&query).await?;
            connection.query(&stmt, &[&offset, &limit]).await?
        };
        for row in result {
            let configuration: Value = row.get("configuration");
            plans.push(from_value(configuration)?);
        }
        Ok(plans)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_queues(&self) -> Result<Vec<String>, Error> {
        let mut plans = Vec::new();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select distinct queue from workflow_plans order by queue desc").await?;
        let result = connection.query(&stmt, &[]).await?;
        for row in result {
            let queue: String = row.get("queue");
            plans.push(queue);
        }
        Ok(plans)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_failed_ids(&self) -> Result<Vec<WorkflowJobId>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select id, queue, configuration->'failed' as failed from workflow_plans where finished is null and (jsonb_array_length(configuration->'failed') > 0)").await?;
        let result = connection.query(&stmt, &[]).await?;
        let mut ids = Vec::new();
        for row in result {
            let id: Uuid = row.get("id");
            let queue: String = row.get("queue");
            let job_ids: Value = row.get("failed");
            for job_id in job_ids.as_array().unwrap() {
                ids.push(WorkflowJobId {
                    id,
                    queue: queue.to_owned(),
                    index: job_id.as_i64().unwrap() as i32,
                })
            }
        }
        Ok(ids)
    }

    #[tracing::instrument(skip(self, ids))]
    pub async fn retry_jobs(&self, ids: Vec<WorkflowJobId>) -> Result<(), Error> {
        let mut redis_txn = RedisTransaction::new();
        let mut conn = self.pool.get().await?;
        let db_txn = conn.transaction().await?;
        for id in ids {
            if let Some(mut plan) = self.get_plan_and_lock_by_job(&db_txn, &id).await? {
                plan.set_job_delayed_until(&id, &db_txn, &mut redis_txn, self, Utc::now()).await?;
                self.set_plan(&db_txn, &plan, false).await?
            }
        }
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, time))]
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
            let key1 = JobQueues::pending_job_queue_key(queue);
            let key2 = JobQueues::running_job_queue_key(queue);
            let result: i32 = script
                .key(key1)
                .key(key2)
                .arg(time)
                .invoke_async(&mut connection)
                .await?;
            if result > 0 {
                info!("found expired jobs: {result}");
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, plan))]
    pub async fn enqueue_plan(
        &self,
        plan: &mut WorkflowExecutionPlan,
    ) -> Result<Option<WorkflowExecutionId>, Error> {
        if plan.finished.is_some() {
            return Err(Error::new("can't enqueue plan, it's already finished"));
        }
        if plan.jobs.is_empty() {
            return Ok(None);
        }
        debug!(target: "workflow", "enqueuing plan: {}", plan.id);
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;
        let mut redis_txn = RedisTransaction::new();
        let state = plan.enqueue(&db_txn, &mut redis_txn, self, 1).await?;
        let mut checkin = true;
        if state == WorkflowExecutePlanState::Complete {
            // return Err(Error::new("can't enqueue plan, it's already complete"));
            warn!("plan is already complete");
            checkin = false;
        }
        if state == WorkflowExecutePlanState::Error {
            return Err(Error::new("can't enqueue plan, it has a state error"));
        }
        if checkin {
            redis_txn.add_op(RedisTransactionOp::PlanCheckin(plan.id.clone()));
        }
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        self.incr("queue::enqueued::count").await?;
        debug!("enqueued plan: {}", plan.id);
        if let Some(id) = &plan.collection_id {
            self.notifier.collection_changed(id).await?;
        }
        if let Some(id) = &plan.metadata_id {
            self.notifier.metadata_changed(id).await?;
        }
        Ok(Some(plan.id.clone()))
    }

    #[tracing::instrument(skip(self, job_id, plans))]
    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowJobId,
        plans: &[WorkflowExecutionPlan],
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        debug!(target: "workflow", "enqueuing job children: {}", job_id);
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;

        let Some(mut parent_plan) = self.get_plan_and_lock_by_job(&db_txn, job_id).await? else {
            return Err(Error::new("can't enqueue child workflows, missing job"));
        };
        let parent_job = parent_plan.jobs.get_mut(job_id.index as usize).unwrap();
        if parent_job.complete {
            db_txn.rollback().await?;
            return Err(Error::new("job is already complete"));
        }

        let mut ids = Vec::new();
        let mut redis_txn = RedisTransaction::new();
        let mut plans = plans.to_vec();
        let mut collection_ids = HashSet::new();
        let mut metadata_ids = HashSet::new();
        for plan in plans.iter_mut() {
            if plan.finished.is_some() {
                return Err(Error::new("can't enqueue plan, it's already finished"));
            }
            plan.parent = Some(parent_job.id.clone());
            let state = plan.enqueue(&db_txn, &mut redis_txn, self, 1).await?;
            if state == WorkflowExecutePlanState::Complete {
                // db_txn.rollback().await?;
                // return Err(Error::new("can't enqueue plan, it's already complete"));
                warn!("plan is already complete");
            }
            if state == WorkflowExecutePlanState::Error {
                db_txn.rollback().await?;
                return Err(Error::new("can't enqueue plan, it has a state error"));
            }
            parent_job.children.insert(plan.id.clone());
            ids.push(plan.id.clone());
            if let Some(id) = &plan.collection_id {
                collection_ids.insert(*id);
            }
            if let Some(id) = &plan.metadata_id {
                metadata_ids.insert(*id);
            }
            self.incr("queue::enqueued::child::count").await?;
            debug!("enqueued plan: {}", plan.id);
        }
        self.set_plan(&db_txn, &parent_plan, false).await?;
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

    #[tracing::instrument(skip(self, pending_key, running_key))]
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

    #[tracing::instrument(skip(self, queue))]
    async fn dequeue_job(&self, queue: &str) -> Result<Option<WorkflowJobId>, Error> {
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
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self, queue))]
    pub async fn dequeue(&self, queue: &str) -> Result<Option<WorkflowJob>, Error> {
        let Some(job_id) = self.dequeue_job(queue).await? else {
            return Ok(None);
        };
        match self.get_plan_by_job(&job_id).await {
            Ok(Some(mut plan)) => {
                if plan.finished.is_some() {
                    error!("invalid plan state");
                    let mut txn = RedisTransaction::new();
                    for job in plan.jobs.iter() {
                        txn.add_op(RemoveJobRunning(job.id.clone()));
                    }
                    txn.add_op(RemovePlanRunning(plan.id.clone()));
                    if let Some(metadata_id) = plan.metadata_id {
                        txn.add_op(RedisTransactionOp::RemoveMetadataRunning(metadata_id));
                    }
                    if let Some(collection_id) = plan.collection_id {
                        txn.add_op(RedisTransactionOp::RemoveCollectionRunning(collection_id));
                    }
                    txn.execute(&self.redis).await?;
                    return Ok(None);
                }
                let mut job = plan.jobs.get_mut(job_id.index as usize).unwrap().clone();
                if job.complete {
                    error!("invalid job state");
                    let mut txn = RedisTransaction::new();
                    txn.add_op(RemoveJobRunning(job.id.clone()));
                    txn.execute(&self.redis).await?;
                    return Ok(None);
                }
                job.parent = plan.parent;
                Ok(Some(job))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                if e.message == "plan not found" {
                    error!("plan not found: {job_id}");
                    let plan = WorkflowExecutionId {
                        id: job_id.id,
                        queue: job_id.queue.clone(),
                    };
                    let mut txn = RedisTransaction::new();
                    txn.add_op(RemoveJobRunning(job_id));
                    txn.add_op(RemovePlanRunning(plan.clone()));
                    txn.execute(&self.redis).await?;
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tracing::instrument(skip(self, plan_id, context))]
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
        plan.context = if context.is_null() {
            None
        } else {
            Some(context.clone())
        };
        self.set_plan(&transaction, &plan, false).await?;
        transaction.commit().await?;
        self.incr("queue::context::set::count").await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id, context))]
    pub async fn set_execution_plan_job_context(
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
        job.context = if context.is_null() {
            None
        } else {
            Some(context.clone())
        };
        if let Some(plan_job) = plan.jobs.get_mut(job_id.index as usize) {
            plan_job.context = if context.is_null() {
                None
            } else {
                Some(context.clone())
            };
        }
        self.set_plan(&transaction, &plan, false).await?;
        transaction.commit().await?;
        self.incr("queue::context::job::set::count").await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id, delayed_until))]
    pub async fn set_execution_plan_job_delayed(
        &self,
        job_id: &WorkflowJobId,
        delayed_until: DateTime<Utc>,
    ) -> Result<WorkflowExecutionPlan, Error> {
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&db_txn, job_id).await? else {
            return Err(Error::new("can't set job context, missing plan"));
        };
        let mut redis_txn = RedisTransaction::new();
        plan.set_job_delayed_until(job_id, &db_txn, &mut redis_txn, self, delayed_until)
            .await?;
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        self.incr("queue::job::delayed").await?;
        Ok(plan)
    }

    #[tracing::instrument(skip(self, job_id, error, try_again))]
    pub async fn set_execution_plan_job_failed(
        &self,
        job_id: &WorkflowJobId,
        error: &str,
        try_again: bool,
    ) -> Result<WorkflowExecutionPlan, Error> {
        let mut connection = self.pool.get().await?;
        let db_txn = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&db_txn, job_id).await? else {
            return Err(Error::new("can't set job context, missing plan"));
        };
        let mut redis_txn = RedisTransaction::new();
        plan.set_job_failed(job_id, &db_txn, &mut redis_txn, self, error, try_again)
            .await?;
        db_txn.commit().await?;
        redis_txn.execute(&self.redis).await?;
        self.incr("queue::job::failed").await?;
        Ok(plan)
    }

    #[tracing::instrument(skip(self, job_id))]
    pub async fn set_execution_plan_job_checkin(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let Some(plan) = self.get_plan_by_job(job_id).await? else {
            return Err(Error::new("can't mark execution complete, missing job"));
        };
        if plan.finished.is_some() {
            return Ok(());
        }
        if let Some(job) = plan.jobs.get(job_id.index as usize) {
            if job.complete {
                return Ok(());
            }
        }
        let mut txn = RedisTransaction::new();
        txn.add_op(JobCheckin(job_id.clone()));
        txn.execute(&self.redis).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id))]
    pub async fn set_execution_plan_job_complete(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<WorkflowExecutionPlan, Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let Some(mut plan) = self.get_plan_and_lock_by_job(&transaction, job_id).await? else {
            return Err(Error::new("can't mark execution complete, missing job"));
        };
        let mut redis_txn = RedisTransaction::new();
        match plan
            .try_set_job_complete(&transaction, &mut redis_txn, self, job_id)
            .await
        {
            Ok(result) => {
                transaction.commit().await?;
                redis_txn.execute(&self.redis).await?;
                self.incr("queue::job::complete").await?;
                if result == WorkflowExecutePlanState::Error {
                    return Err(Error::new("plan is in an error state"));
                }
                Ok(plan)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }
}
