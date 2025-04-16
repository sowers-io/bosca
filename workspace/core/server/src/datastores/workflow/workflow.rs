use crate::context::BoscaContext;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::notifier::Notifier;
use crate::datastores::workflow::workflow_cache::WorkflowCache;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::workflow::activities::{
    Activity, ActivityInput, ActivityParameter, WorkflowActivity, WorkflowActivityInput,
    WorkflowActivityModel, WorkflowActivityParameter, WorkflowActivityPrompt,
    WorkflowActivityStorageSystem,
};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::{
    WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJob, WorkflowJobId,
};
use crate::models::workflow::models::{Model, ModelInput};
use crate::models::workflow::prompts::{Prompt, PromptInput};
use crate::models::workflow::states::{WorkflowState, WorkflowStateInput};
use crate::models::workflow::storage_system_models::{StorageSystemModel, StorageSystemModelInput};
use crate::models::workflow::storage_systems::{StorageSystem, StorageSystemInput};
use crate::models::workflow::traits::{Trait, TraitInput};
use crate::models::workflow::transitions::{Transition, TransitionInput};
use crate::models::workflow::workflows::{Workflow, WorkflowInput};
use crate::util::RUNNING_BACKGROUND;
use crate::workflow::core_workflow_ids::STORAGE_INDEX_INITIALIZE;
use crate::workflow::queue::JobQueues;
use async_graphql::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::{error, info, warn};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkflowDataStore {
    pool: Arc<Pool>,
    queues: JobQueues,
    cache: WorkflowCache,
    notifier: Arc<Notifier>,
}

impl WorkflowDataStore {
    pub async fn new(
        pool: Arc<Pool>,
        cache: &mut BoscaCacheManager,
        queues: JobQueues,
        notifier: Arc<Notifier>,
    ) -> Self {
        Self {
            pool,
            queues,
            cache: WorkflowCache::new(cache).await,
            notifier,
        }
    }

    pub fn start_monitoring_expirations(&self) {
        info!("starting background monitoring of workflow expiration");
        let jobs_expiration = self.queues.clone();
        tokio::task::spawn(async move {
            loop {
                RUNNING_BACKGROUND.fetch_add(1, Relaxed);
                let now = Utc::now().timestamp();
                if let Err(e) = jobs_expiration.check_for_expiration(now).await {
                    error!(target: "workflow", "failed to check for expiration: {:?}", e);
                }
                RUNNING_BACKGROUND.fetch_add(-1, Relaxed);
                sleep(Duration::from_secs(3)).await;
            }
        });
    }

    /* activities */

    #[tracing::instrument(skip(self, activity))]
    pub async fn add_activity(&self, activity: &ActivityInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into activities (id, name, description, child_workflow_id, configuration) values ($1, $2, $3, $4, $5)").await?;
        connection
            .query(
                &stmt,
                &[
                    &activity.id,
                    &activity.name,
                    &activity.description,
                    &activity.child_workflow_id,
                    &activity.configuration,
                ],
            )
            .await?;
        drop(stmt);
        let stmt = connection
            .prepare_cached(
                "insert into activity_inputs (activity_id, name, type) values ($1, $2, $3)",
            )
            .await?;
        for input in activity.inputs.iter() {
            connection
                .execute(&stmt, &[&activity.id, &input.name, &input.parameter_type])
                .await?;
        }
        drop(stmt);
        let stmt = connection
            .prepare_cached(
                "insert into activity_outputs (activity_id, name, type) values ($1, $2, $3)",
            )
            .await?;
        for input in activity.outputs.iter() {
            connection
                .execute(&stmt, &[&activity.id, &input.name, &input.parameter_type])
                .await?;
        }
        self.notifier.activity_changed(&activity.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, activity))]
    pub async fn edit_activity(&self, activity: &ActivityInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update activities set name = $2, description = $3, child_workflow_id = $4, configuration = $5 where id = $1").await?;
        txn.query(
            &stmt,
            &[
                &activity.id,
                &activity.name,
                &activity.description,
                &activity.child_workflow_id,
                &activity.configuration,
            ],
        )
        .await?;
        txn.query(
            "delete from activity_inputs where activity_id = $1",
            &[&activity.id],
        )
        .await?;
        txn.query(
            "delete from activity_outputs where activity_id = $1",
            &[&activity.id],
        )
        .await?;
        let stmt = txn
            .prepare_cached(
                "insert into activity_inputs (activity_id, name, type) values ($1, $2, $3)",
            )
            .await?;
        for input in activity.inputs.iter() {
            txn.execute(&stmt, &[&activity.id, &input.name, &input.parameter_type])
                .await?;
        }
        let stmt = txn
            .prepare_cached(
                "insert into activity_outputs (activity_id, name, type) values ($1, $2, $3)",
            )
            .await?;
        for input in activity.outputs.iter() {
            txn.execute(&stmt, &[&activity.id, &input.name, &input.parameter_type])
                .await?;
        }
        txn.commit().await?;
        self.notifier.activity_changed(&activity.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn delete_activity(&self, activity_id: &str) -> Result<(), Error> {
        let activity_id = activity_id.to_owned();
        let connection = self.pool.get().await?;
        connection
            .execute("delete from activities where id = $1", &[&activity_id])
            .await?;
        self.cache.evict_activity(&activity_id).await;
        self.notifier.activity_changed(&activity_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_activities(&self) -> Result<Vec<Activity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activities")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(Activity::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_activity(&self, id: &String) -> Result<Option<Activity>, Error> {
        if let Some(activity) = self.cache.get_activity(id).await {
            return Ok(Some(activity));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activities where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let activity = rows.first().unwrap().into();
        self.cache.set_activity(&activity).await;
        Ok(Some(activity))
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_activity_inputs(
        &self,
        activity_id: &String,
    ) -> Result<Vec<ActivityParameter>, Error> {
        if let Some(activity) = self.cache.get_activity_inputs(activity_id).await {
            return Ok(activity);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activity_inputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let parameters = rows.iter().map(ActivityParameter::from).collect();
        self.cache
            .set_activity_inputs(activity_id, &parameters)
            .await;
        Ok(parameters)
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_activity_outputs(
        &self,
        activity_id: &String,
    ) -> Result<Vec<ActivityParameter>, Error> {
        if let Some(activity) = self.cache.get_activity_outputs(activity_id).await {
            return Ok(activity);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activity_outputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let parameters = rows.iter().map(ActivityParameter::from).collect();
        self.cache
            .set_activity_outputs(activity_id, &parameters)
            .await;
        Ok(parameters)
    }

    /* activities */

    /* workflows */

    #[tracing::instrument(skip(self))]
    pub async fn get_workflows(&self) -> Result<Vec<Workflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from workflows").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Workflow::from).collect())
    }

    #[tracing::instrument(skip(self, trait_id))]
    pub async fn get_workflows_by_trait(&self, trait_id: &String) -> Result<Vec<Workflow>, Error> {
        if let Some(workflow_ids) = self.cache.get_trait_workflow_ids(trait_id).await {
            let mut workflows = Vec::new();
            for id in workflow_ids {
                if let Some(workflow) = self.get_workflow(&id).await? {
                    workflows.push(workflow);
                }
            }
            return Ok(workflows);
        }
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select w.id as id from workflows w inner join trait_workflows tw on (tw.workflow_id = w.id and tw.trait_id = $1)").await?;
        let rows = connection.query(&stmt, &[trait_id]).await?;
        let mut workflow_ids = Vec::new();
        let mut workflows = Vec::new();
        for row in rows.iter() {
            let id: String = row.get(0);
            if let Some(workflow) = self.get_workflow(&id).await? {
                workflows.push(workflow);
            }
            workflow_ids.push(id);
        }
        self.cache
            .set_trait_workflow_ids(trait_id, &workflow_ids)
            .await;
        Ok(workflows)
    }

    #[tracing::instrument(skip(self, workflow))]
    pub async fn add_workflow(&self, workflow: &WorkflowInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        self.add_workflow_txn(&txn, workflow).await?;
        txn.commit().await?;
        self.notifier.workflow_changed(&workflow.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, workflow))]
    async fn add_workflow_txn(
        &self,
        txn: &Transaction<'_>,
        workflow: &WorkflowInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into workflows (id, name, description, queue, configuration) values ($1, $2, $3, $4, $5) returning id").await?;
        txn.query(
            &stmt,
            &[
                &workflow.id,
                &workflow.name,
                &workflow.description,
                &workflow.queue,
                &workflow.configuration,
            ],
        )
        .await?;
        for activity in &workflow.activities {
            self.add_workflow_activity(txn, &workflow.id, activity)
                .await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, workflow))]
    pub async fn edit_workflow(&self, workflow: &WorkflowInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        self.delete_workflow_txn(&txn, &workflow.id, false).await?;
        let stmt = txn.prepare_cached("update workflows set name = $2, description = $3, queue = $4, configuration = $5 where id = $1").await?;
        txn.query(
            &stmt,
            &[
                &workflow.id,
                &workflow.name,
                &workflow.description,
                &workflow.queue,
                &workflow.configuration,
            ],
        )
        .await?;
        let mut ids = Vec::new();
        for activity in &workflow.activities {
            let id = self
                .add_workflow_activity(&txn, &workflow.id, activity)
                .await?;
            ids.push(id);
        }
        txn.commit().await?;

        for id in ids {
            self.notifier.workflow_activity_changed(id).await?;
            self.cache.evict_workflow_activity(&id).await;
        }

        self.cache.evict_workflow(&workflow.id).await;
        self.notifier.workflow_changed(&workflow.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, include_workflow))]
    async fn delete_workflow_txn(
        &self,
        txn: &Transaction<'_>,
        id: &String,
        include_workflow: bool,
    ) -> Result<(), Error> {
        txn.execute("delete from workflow_activity_inputs where activity_id in (select id from workflow_activities where workflow_id = $1)", &[id]).await?;
        txn.execute("delete from workflow_activity_outputs where activity_id in (select id from workflow_activities where workflow_id = $1)", &[id]).await?;
        txn.execute("delete from workflow_activity_models where activity_id in (select id from workflow_activities where workflow_id = $1)", &[id]).await?;
        txn.execute("delete from workflow_activity_prompts where activity_id in (select id from workflow_activities where workflow_id = $1)", &[id]).await?;
        txn.execute("delete from workflow_activity_storage_systems where activity_id in (select id from workflow_activities where workflow_id = $1)", &[id]).await?;
        txn.execute(
            "delete from workflow_activities where workflow_id = $1",
            &[id],
        )
        .await?;
        if include_workflow {
            txn.execute("delete from workflows where id = $1", &[id])
                .await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_workflow(&self, id: &str) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let id = id.to_owned();
        self.delete_workflow_txn(&txn, &id, true).await?;
        txn.commit().await?;
        self.cache.evict_workflow(&id).await;
        self.notifier.workflow_changed(&id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_metadata_count(&self, id: &Uuid) -> Result<i64, Error> {
        self.queues.get_metadata_count(id).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_collection_count(&self, id: &Uuid) -> Result<i64, Error> {
        self.queues.get_collection_count(id).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_workflow(&self, id: &str) -> Result<Option<Workflow>, Error> {
        let id = id.to_owned();
        if let Some(workflow) = self.cache.get_workflow(&id).await {
            return Ok(Some(workflow));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflows where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[&id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let workflow = rows.remove(0).into();
        self.cache.set_workflow(&workflow).await;
        Ok(Some(workflow))
    }

    /* workflows */

    /* workflow activities */

    #[tracing::instrument(skip(self, txn, workflow_id, activity))]
    async fn add_workflow_activity(
        &self,
        txn: &Transaction<'_>,
        workflow_id: &str,
        activity: &WorkflowActivityInput,
    ) -> Result<i64, Error> {
        let execution_group = if activity.execution_group == 0 {
            1
        } else {
            activity.execution_group
        };
        let workflow_id = workflow_id.to_owned();
        let mut configuration = activity
            .configuration
            .as_ref()
            .unwrap_or(&Value::Null)
            .clone();
        if configuration.is_null() {
            configuration = Value::Object(serde_json::Map::new());
        }
        let id: i64 = {
            let stmt = txn.prepare_cached("insert into workflow_activities (workflow_id, activity_id, execution_group, queue, configuration) values ($1, $2, $3, $4, $5) returning id").await?;
            let rows = txn
                .query(
                    &stmt,
                    &[
                        &workflow_id,
                        &activity.activity_id,
                        &execution_group,
                        &activity.queue,
                        &configuration,
                    ],
                )
                .await?;
            rows.first().unwrap().get(0)
        };
        {
            let stmt = txn.prepare_cached("insert into workflow_activity_inputs (activity_id, name, value) values ($1, $2, $3)").await?;
            for input in activity.inputs.iter() {
                txn.execute(&stmt, &[&id, &input.name, &input.value])
                    .await?;
            }
        }
        {
            let stmt = txn.prepare_cached("insert into workflow_activity_outputs (activity_id, name, value) values ($1, $2, $3)").await?;
            for input in activity.outputs.iter() {
                txn.execute(&stmt, &[&id, &input.name, &input.value])
                    .await?;
            }
        }
        {
            let stmt = txn.prepare_cached("insert into workflow_activity_models (activity_id, model_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.models.iter() {
                let mid = Uuid::parse_str(input.model_id.as_str())?;
                txn.execute(&stmt, &[&id, &mid, &input.configuration])
                    .await?;
            }
        }
        {
            let stmt = txn.prepare_cached("insert into workflow_activity_prompts (activity_id, prompt_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.prompts.iter() {
                let pid = Uuid::parse_str(input.prompt_id.as_str())?;
                txn.execute(&stmt, &[&id, &pid, &input.configuration])
                    .await?;
            }
        }
        {
            let stmt = txn.prepare_cached("insert into workflow_activity_storage_systems (activity_id, storage_system_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.storage_systems.iter() {
                let sid = Uuid::parse_str(input.system_id.as_str())?;
                txn.execute(&stmt, &[&id, &sid, &input.configuration])
                    .await?;
            }
        }
        self.notifier.workflow_activity_changed(id).await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity(
        &self,
        activity_id: &i64,
    ) -> Result<Option<WorkflowActivity>, Error> {
        if let Some(workflow_activity) = self.cache.get_workflow_activity(activity_id).await {
            return Ok(Some(workflow_activity));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activities where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        if let Some(activity) = rows.first().map(WorkflowActivity::from) {
            self.cache.set_workflow_activity(&activity).await;
            return Ok(Some(activity));
        }
        Ok(None)
    }

    #[tracing::instrument(skip(self, workflow_id))]
    pub async fn get_workflow_activities(
        &self,
        workflow_id: &String,
    ) -> Result<Vec<WorkflowActivity>, Error> {
        if let Some(ids) = self.cache.get_workflow_activity_ids(workflow_id).await {
            let mut activities = Vec::new();
            for id in ids {
                if let Some(activity) = self.get_workflow_activity(&id).await? {
                    activities.push(activity);
                }
            }
            return Ok(activities);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select id from workflow_activities where workflow_id = $1 order by execution_group asc")
            .await?;
        let rows = connection.query(&stmt, &[workflow_id]).await?;
        let mut activities = Vec::new();
        let mut activity_ids = Vec::new();
        for row in rows.iter() {
            let id: i64 = row.get(0);
            if let Some(activity) = self.get_workflow_activity(&id).await? {
                activities.push(activity);
                activity_ids.push(id);
            }
        }
        self.cache
            .set_workflow_activity_ids(workflow_id, &activity_ids)
            .await;
        Ok(activities)
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity_inputs(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityParameter>, Error> {
        if let Some(inputs) = self.cache.get_workflow_activity_inputs(activity_id).await {
            return Ok(inputs);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_inputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let inputs = rows.iter().map(WorkflowActivityParameter::from).collect();
        self.cache
            .set_workflow_activity_inputs(activity_id, &inputs)
            .await;
        Ok(inputs)
    }

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity_outputs(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityParameter>, Error> {
        if let Some(outputs) = self.cache.get_workflow_activity_outputs(activity_id).await {
            return Ok(outputs);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_outputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let outputs = rows.iter().map(WorkflowActivityParameter::from).collect();
        self.cache
            .set_workflow_activity_outputs(activity_id, &outputs)
            .await;
        Ok(outputs)
    }

    /* workflow activities */

    /* workflow activity models */

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity_models(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityModel>, Error> {
        if let Some(models) = self.cache.get_workflow_activity_models(activity_id).await {
            return Ok(models);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_models where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let models = rows.iter().map(WorkflowActivityModel::from).collect();
        self.cache
            .set_workflow_activity_models(activity_id, &models)
            .await;
        Ok(models)
    }

    /* workflow activity models */

    /* workflow activity prompts */

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity_prompts(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityPrompt>, Error> {
        if let Some(prompts) = self.cache.get_workflow_activity_prompts(activity_id).await {
            return Ok(prompts);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_prompts where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let prompts = rows.iter().map(WorkflowActivityPrompt::from).collect();
        self.cache
            .set_workflow_activity_prompts(activity_id, &prompts)
            .await;
        Ok(prompts)
    }

    /* workflow activity prompts */

    /* workflow activity storage systems */

    #[tracing::instrument(skip(self, activity_id))]
    pub async fn get_workflow_activity_storage_systems(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityStorageSystem>, Error> {
        if let Some(systems) = self
            .cache
            .get_workflow_activity_storage_systems(activity_id)
            .await
        {
            return Ok(systems);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from workflow_activity_storage_systems where activity_id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        let systems = rows
            .iter()
            .map(WorkflowActivityStorageSystem::from)
            .collect();
        self.cache
            .set_workflow_activity_storage_systems(activity_id, &systems)
            .await;
        Ok(systems)
    }

    /* workflow activity storage systems */

    /* models */

    #[tracing::instrument(skip(self))]
    pub async fn get_models(&self) -> Result<Vec<Model>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from models").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Model::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_model(&self, id: &Uuid) -> Result<Option<Model>, Error> {
        if let Some(model) = self.cache.get_model(id).await {
            return Ok(Some(model));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from models where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let model = rows.remove(0).into();
        self.cache.set_model(&model).await;
        Ok(Some(model))
    }

    #[tracing::instrument(skip(self, model))]
    pub async fn add_model(&self, model: &ModelInput) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into models (type, name, description, configuration) values ($1, $2, $3, $4) returning id").await?;
        let rows = connection
            .query(
                &stmt,
                &[
                    &model.model_type,
                    &model.name,
                    &model.description,
                    &model.configuration,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(Uuid::nil());
        }
        let id: Uuid = rows.first().unwrap().get(0);
        let id_str = id.to_string();
        self.notifier.model_changed(&id_str).await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, id, model))]
    pub async fn edit_model(&self, id: &Uuid, model: &ModelInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update models set type = $1, name = $2, description = $3, configuration = $4 where id = $5").await?;
        connection
            .execute(
                &stmt,
                &[
                    &model.model_type,
                    &model.name,
                    &model.description,
                    &model.configuration,
                    &id,
                ],
            )
            .await?;
        let id_str = id.to_string();
        self.cache.evict_model(id).await;
        self.notifier.model_changed(&id_str).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_model(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from models where id = $5")
            .await?;
        connection.execute(&stmt, &[&id]).await?;
        self.cache.evict_model(id).await;
        let id_str = id.to_string();
        self.notifier.model_changed(&id_str).await?;
        Ok(())
    }

    /* models */

    /* states */

    #[tracing::instrument(skip(self))]
    pub async fn get_states(&self) -> Result<Vec<WorkflowState>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_states")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(WorkflowState::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_state(&self, id: &str) -> Result<Option<WorkflowState>, Error> {
        let id = id.to_owned();
        if let Some(state) = self.cache.get_state(&id).await {
            return Ok(Some(state));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_states where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[&id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let state = rows.remove(0).into();
        self.cache.set_state(&state).await;
        Ok(Some(state))
    }

    #[tracing::instrument(skip(self, state))]
    pub async fn add_state(&self, state: &WorkflowStateInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into workflow_states (id, type, name, description, configuration, workflow_id, entry_workflow_id, exit_workflow_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        connection
            .query(
                &stmt,
                &[
                    &state.id,
                    &state.state_type,
                    &state.name,
                    &state.description,
                    &state.configuration,
                    &state.workflow_id,
                    &state.entry_workflow_id,
                    &state.exit_workflow_id,
                ],
            )
            .await?;
        self.notifier.state_changed(&state.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, state))]
    pub async fn edit_state(&self, state: &WorkflowStateInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update workflow_states set type = $2, name = $3, description = $4, configuration = $5, workflow_id = $6, entry_workflow_id = $7, exit_workflow_id = $8 where id = $1").await?;
        connection
            .query(
                &stmt,
                &[
                    &state.id,
                    &state.state_type,
                    &state.name,
                    &state.description,
                    &state.configuration,
                    &state.workflow_id,
                    &state.entry_workflow_id,
                    &state.exit_workflow_id,
                ],
            )
            .await?;
        self.cache.evict_state(&state.id).await;
        self.notifier.state_changed(&state.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_state(&self, id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from workflow_states where id = $1")
            .await?;
        connection.execute(&stmt, &[&id]).await?;
        self.cache.evict_state(id).await;
        self.notifier.state_changed(id).await?;
        Ok(())
    }

    /* states */

    /* storage systems */

    #[tracing::instrument(skip(self))]
    pub async fn get_storage_systems(&self) -> Result<Vec<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(StorageSystem::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_storage_system(&self, id: &Uuid) -> Result<Option<StorageSystem>, Error> {
        if let Some(system) = self.cache.get_storage_system(id).await {
            return Ok(Some(system));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let system = rows.first().map(|r| r.into());
        if let Some(system) = &system {
            self.cache.set_storage_system(system).await;
        }
        Ok(system)
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn get_storage_system_by_name(
        &self,
        name: &str,
    ) -> Result<Option<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let name = name.to_string();
        let stmt = connection
            .prepare_cached("select * from storage_systems where name = $1")
            .await?;
        let rows = connection.query(&stmt, &[&name]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, ctx, system))]
    pub async fn add_storage_system(
        &self,
        ctx: &BoscaContext,
        system: &StorageSystemInput,
    ) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let mut txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into storage_systems (type, name, description, configuration) values ($1, $2, $3, $4) returning id").await?;
        let rows = txn
            .query(
                &stmt,
                &[
                    &system.system_type,
                    &system.name,
                    &system.description,
                    &system.configuration,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(Uuid::nil());
        }
        let id = rows.first().unwrap().get(0);
        for model in system.models.iter() {
            self.add_storage_system_model_txn(&mut txn, &id, model)
                .await?;
        }
        txn.commit().await?;

        let mut request = EnqueueRequest {
            workflow_id: Some(STORAGE_INDEX_INITIALIZE.to_string()),
            storage_system_ids: Some(vec![id]),
            ..Default::default()
        };
        if let Err(e) = ctx.workflow.enqueue_workflow(ctx, &mut request).await {
            log::error!("Failed to initialize storage system: {:?}", e);
        }

        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;

        Ok(id)
    }

    #[tracing::instrument(skip(self, ctx, id, system))]
    pub async fn edit_storage_system(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        system: &StorageSystemInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let mut txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update storage_systems set type = $1, name = $2, description = $3, configuration = $4 where id = $5").await?;
        txn.execute(
            &stmt,
            &[
                &system.system_type,
                &system.name,
                &system.description,
                &system.configuration,
                &id,
            ],
        )
        .await?;
        let stmt = txn
            .prepare_cached("delete from storage_system_models where system_id = $1")
            .await?;
        txn.execute(&stmt, &[&id]).await?;
        for model in system.models.iter() {
            self.add_storage_system_model_txn(&mut txn, id, model)
                .await?;
        }
        txn.commit().await?;

        self.cache.evict_storage_system(id).await;

        let mut request = EnqueueRequest {
            workflow_id: Some(STORAGE_INDEX_INITIALIZE.to_string()),
            storage_system_ids: Some(vec![*id]),
            ..Default::default()
        };
        if let Err(e) = ctx.workflow.enqueue_workflow(ctx, &mut request).await {
            log::error!("Failed to initialize storage system: {:?}", e);
        }

        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;

        Ok(())
    }

    /* storage systems */

    /* storage system models */

    #[tracing::instrument(skip(self, txn, system_id, model))]
    async fn add_storage_system_model_txn(
        &self,
        txn: &mut Transaction<'_>,
        system_id: &Uuid,
        model: &StorageSystemModelInput,
    ) -> Result<(), Error> {
        let model_id = Uuid::parse_str(model.model_id.as_str())?;
        let stmt = txn.prepare_cached("insert into storage_system_models (system_id, model_id, configuration) values ($1, $2, $3)").await?;
        txn.query(&stmt, &[&system_id, &model_id, &model.configuration])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_storage_system_models(
        &self,
        id: &Uuid,
    ) -> Result<Vec<StorageSystemModel>, Error> {
        if let Some(models) = self.cache.get_storage_system_models(id).await {
            return Ok(models);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_system_models where system_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let models = rows.into_iter().map(StorageSystemModel::from).collect();
        self.cache.set_storage_system_models(id, &models).await;
        Ok(models)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_storage_system(&self, id: &Uuid) -> Result<bool, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from storage_system where idf = $1")
            .await?;
        connection.query(&stmt, &[id]).await?;
        self.cache.evict_storage_system(id).await;
        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;
        Ok(true)
    }

    /* storage system models */

    /* prompts */

    #[tracing::instrument(skip(self, prompt))]
    pub async fn add_prompt(&self, prompt: &PromptInput) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into prompts (name, description, system_prompt, user_prompt, input_type, output_type, schema) values ($1, $2, $3, $4, $5, $6, $7) returning id").await?;
        let rows = connection
            .query(
                &stmt,
                &[
                    &prompt.name,
                    &prompt.description,
                    &prompt.system_prompt,
                    &prompt.user_prompt,
                    &prompt.input_type,
                    &prompt.output_type,
                    &prompt.schema,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(Uuid::nil());
        }
        let id: Uuid = rows.first().unwrap().get(0);
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, id, prompt))]
    pub async fn edit_prompt(&self, id: &Uuid, prompt: &PromptInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update prompts set name = $1, description = $2, system_prompt = $3, user_prompt = $4, input_type = $5, output_type = $6, schema = $7 where id = $8").await?;
        connection
            .execute(
                &stmt,
                &[
                    &prompt.name,
                    &prompt.description,
                    &prompt.system_prompt,
                    &prompt.user_prompt,
                    &prompt.input_type,
                    &prompt.output_type,
                    &prompt.schema,
                    id,
                ],
            )
            .await?;
        self.cache.evict_prompt(id).await;
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_prompt(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from prompts where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.cache.evict_prompt(id).await;
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_prompts(&self) -> Result<Vec<Prompt>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from prompts").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(Prompt::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_prompt(&self, id: &Uuid) -> Result<Option<Prompt>, Error> {
        if let Some(prompt) = self.cache.get_prompt(id).await {
            return Ok(Some(prompt));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from prompts where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let prompt = rows.first().unwrap().into();
        self.cache.set_prompt(&prompt).await;
        Ok(Some(prompt))
    }

    /* prompts */

    /* traits */

    #[tracing::instrument(skip(self))]
    pub async fn get_traits(&self) -> Result<Vec<Trait>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from traits order by name asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        let mut traits = Vec::<Trait>::new();
        let workflow_stmt = connection
            .prepare_cached("select workflow_id from trait_workflows where trait_id = $1")
            .await?;
        let type_stmt = connection
            .prepare_cached("select content_type from trait_content_types where trait_id = $1")
            .await?;
        for row in rows.iter() {
            let mut t: Trait = row.into();
            let rows = connection.query(&workflow_stmt, &[&t.id]).await?;
            t.workflow_ids = rows
                .iter()
                .map(|r| r.get::<&str, String>("workflow_id").to_string())
                .collect();
            let rows = connection.query(&type_stmt, &[&t.id]).await?;
            t.content_types = rows
                .iter()
                .map(|r| r.get::<&str, String>("content_type").to_string())
                .collect();
            traits.push(t);
        }
        Ok(traits)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_trait(&self, id: &String) -> Result<Option<Trait>, Error> {
        if let Some(t) = self.cache.get_trait(id).await {
            return Ok(Some(t));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from traits where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let id_stmt = connection
            .prepare_cached("select workflow_id from trait_workflows where trait_id = $1")
            .await?;
        let ct_stmt = connection
            .prepare_cached("select content_type from trait_content_types where trait_id = $1")
            .await?;
        if let Some(row) = rows.first() {
            let mut t: Trait = row.into();
            let rows = connection.query(&id_stmt, &[&t.id]).await?;
            t.workflow_ids = rows
                .iter()
                .map(|r| r.get::<&str, String>("workflow_id").to_string())
                .collect();
            let rows = connection.query(&ct_stmt, &[&t.id]).await?;
            t.content_types = rows
                .iter()
                .map(|r| r.get::<&str, String>("content_type").to_string())
                .collect();
            self.cache.set_trait(&t).await;
            return Ok(Some(t));
        }
        Ok(None)
    }

    #[tracing::instrument(skip(self, t))]
    pub async fn add_trait(&self, t: &TraitInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into traits (id, name, description, delete_workflow_id) values ($1, $2, $3, $4)")
            .await?;
        connection
            .query(
                &stmt,
                &[&t.id, &t.name, &t.description, &t.delete_workflow_id],
            )
            .await?;
        drop(stmt);
        let stmt = connection
            .prepare_cached("insert into trait_workflows (trait_id, workflow_id) values ($1, $2)")
            .await?;
        for workflow_id in t.workflow_ids.iter() {
            connection.execute(&stmt, &[&t.id, workflow_id]).await?;
        }
        let stmt = connection
            .prepare_cached(
                "insert into trait_content_types (trait_id, content_type) values ($1, $2)",
            )
            .await?;
        for content_type in t.content_types.iter() {
            connection.execute(&stmt, &[&t.id, content_type]).await?;
        }
        self.notifier.trait_changed(&t.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, t))]
    pub async fn edit_trait(&self, t: &TraitInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let stmt = transaction
            .prepare_cached("update traits set name = $2, description = $3, delete_workflow_id = $4 where id = $1")
            .await?;
        transaction
            .query(
                &stmt,
                &[&t.id, &t.name, &t.description, &t.delete_workflow_id],
            )
            .await?;
        drop(stmt);
        transaction
            .execute("delete from trait_workflows where trait_id = $1", &[&t.id])
            .await?;
        transaction
            .execute(
                "delete from trait_content_types where trait_id = $1",
                &[&t.id],
            )
            .await?;
        let stmt = transaction
            .prepare_cached("insert into trait_workflows (trait_id, workflow_id) values ($1, $2)")
            .await?;
        for workflow_id in t.workflow_ids.iter() {
            transaction.execute(&stmt, &[&t.id, workflow_id]).await?;
        }
        let stmt = transaction
            .prepare_cached(
                "insert into trait_content_types (trait_id, content_type) values ($1, $2)",
            )
            .await?;
        for content_type in t.content_types.iter() {
            transaction.execute(&stmt, &[&t.id, content_type]).await?;
        }
        transaction.commit().await?;
        self.cache.evict_trait(&t.id).await;
        self.notifier.trait_changed(&t.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_trait(&self, id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        connection
            .execute("delete from traits where id = $1", &[&id])
            .await?;
        self.cache.evict_trait(id).await;
        self.notifier.trait_changed(id).await?;
        Ok(())
    }

    /* traits */

    /* transitions */

    #[tracing::instrument(skip(self, t))]
    pub async fn add_transition(&self, t: &TransitionInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into workflow_state_transitions (from_state_id, to_state_id, description) values ($1, $2, $3)").await?;
        connection
            .query(&stmt, &[&t.from_state_id, &t.to_state_id, &t.description])
            .await?;
        self.notifier
            .transition_changed(&t.from_state_id, &t.to_state_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, t))]
    pub async fn edit_transition(&self, t: &TransitionInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update workflow_state_transitions set description = $3 where from_state_id = $1 and to_state_id = $2").await?;
        connection
            .query(&stmt, &[&t.from_state_id, &t.to_state_id, &t.description])
            .await?;
        self.cache
            .evict_transition(&t.from_state_id, &t.to_state_id)
            .await;
        self.notifier
            .transition_changed(&t.from_state_id, &t.to_state_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, from_state_id, to_state_id))]
    pub async fn delete_transition(
        &self,
        from_state_id: &String,
        to_state_id: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete workflow_state_transitions where from_state_id = $1 and to_state_id = $2",
            )
            .await?;
        connection
            .query(&stmt, &[from_state_id, to_state_id])
            .await?;
        self.cache
            .evict_transition(from_state_id, to_state_id)
            .await;
        self.notifier
            .transition_changed(from_state_id, to_state_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_transitions(&self) -> Result<Vec<Transition>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_state_transitions")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Transition::from).collect())
    }

    #[tracing::instrument(skip(self, from_state_id, to_state_id))]
    pub async fn get_transition(
        &self,
        from_state_id: &str,
        to_state_id: &str,
    ) -> Result<Option<Transition>, Error> {
        let from = from_state_id.to_owned();
        let to = to_state_id.to_owned();
        if let Some(t) = self.cache.get_transition(&from, &to).await {
            return Ok(Some(t));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_state_transitions where from_state_id = $1 and to_state_id = $2")
            .await?;
        let rows = connection.query(&stmt, &[&from, &to]).await?;
        if let Some(transition) = rows.into_iter().next().map(Transition::from) {
            self.cache.set_transition(&transition).await;
            return Ok(Some(transition));
        }
        Ok(None)
    }

    /* transitions */

    /* queues */

    #[tracing::instrument(skip(self))]
    pub async fn expire_all(&self) -> Result<(), Error> {
        self.queues.check_for_expiration(i64::MAX).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn retry_all_failed(&self) -> Result<(), Error> {
        let ids = self.queues.get_failed_ids().await?;
        self.queues.retry_jobs(ids).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id, workflow_ids, delay_until))]
    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowJobId,
        workflow_ids: &Vec<String>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut plans = Vec::<WorkflowExecutionPlan>::new();
        let Some(plan) = self.queues.get_plan_by_job(job_id).await? else {
            return Err(Error::new("missing plan"));
        };
        let job = plan.jobs.get(job_id.index as usize).unwrap();
        let mut request = EnqueueRequest {
            collection_id: job
                .collection_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap()),
            metadata_id: job
                .metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap()),
            metadata_version: job.metadata_version,
            profile_id: job
                .profile_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap()),
            delay_until,
            ..Default::default()
        };
        for workflow_id in workflow_ids {
            request.workflow_id = Some(workflow_id.clone());
            let plan = self.get_new_execution_plan(&mut request).await?;
            plans.push(plan);
        }
        self.queues
            .enqueue_job_child_workflows(job_id, &plans)
            .await
    }

    #[tracing::instrument(skip(self, job_id, workflow_id, configurations, delay_until))]
    pub async fn enqueue_job_child_workflow(
        &self,
        job_id: &WorkflowJobId,
        workflow_id: &str,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<WorkflowExecutionId, Error> {
        let workflow_id = workflow_id.to_owned();
        let mut plans = Vec::<WorkflowExecutionPlan>::new();
        let Some(plan) = self.queues.get_plan_by_job(job_id).await? else {
            return Err(Error::new("missing plan"));
        };
        let job = plan.jobs.get(job_id.index as usize).unwrap();
        let metadata_id = job
            .metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let collection_id = job
            .collection_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let mut request = EnqueueRequest {
            workflow_id: Some(workflow_id),
            collection_id,
            metadata_id,
            metadata_version: job.metadata_version,
            configurations,
            delay_until,
            ..Default::default()
        };
        let plan = self.get_new_execution_plan(&mut request).await?;
        plans.push(plan);
        Ok(self
            .queues
            .enqueue_job_child_workflows(job_id, &plans)
            .await?
            .first()
            .unwrap()
            .clone())
    }

    #[tracing::instrument(skip(self, queue))]
    pub async fn dequeue_next_execution(&self, queue: &str) -> Result<Option<WorkflowJob>, Error> {
        self.queues.dequeue(queue).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_execution_plan(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        self.queues.get_plan(id).await
    }

    #[tracing::instrument(skip(self, queue, offset, limit))]
    pub async fn get_execution_plans(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        self.queues.get_all_plans(queue, offset, limit).await
    }

    #[tracing::instrument(skip(self, request))]
    pub async fn get_new_execution_plan(
        &self,
        request: &mut EnqueueRequest,
    ) -> Result<WorkflowExecutionPlan, Error> {
        let workflow = if let Some(workflow) = request.workflow.take() {
            workflow
        } else if let Some(workflow_id) = &request.workflow_id {
            if let Some(workflow) = self.get_workflow(workflow_id).await? {
                workflow
            } else {
                return Err(Error::new(format!("missing workflow: {}", workflow_id)));
            }
        } else {
            return Err(Error::new("workflow or workflow_id is required"));
        };
        let mut jobs = Vec::<WorkflowJob>::new();
        let activities = self.get_workflow_activities(&workflow.id).await?;
        let mut pending = HashSet::<i32>::new();
        let mut current_execution_group = Vec::<i32>::new();
        let mut configuration_overrides = HashMap::new();
        if let Some(overrides) = &request.configurations {
            for o in overrides.iter() {
                configuration_overrides
                    .insert(o.activity_id.to_owned(), o.configuration.to_owned());
            }
        }
        let plan_id = Uuid::new_v4();
        for mut workflow_activity in activities.into_iter() {
            if let Some(Value::Object(o)) =
                configuration_overrides.get(&workflow_activity.activity_id)
            {
                if workflow_activity.configuration.is_none()
                    || workflow_activity.configuration.as_ref().unwrap().is_null()
                {
                    workflow_activity.configuration = Some(Value::Object(o.clone()));
                } else if let Some(Value::Object(ref mut o2)) = workflow_activity.configuration {
                    for (k, v) in o.iter() {
                        o2.insert(k.to_owned(), v.to_owned());
                    }
                }
            }
            let models = self
                .get_workflow_activity_models(&workflow_activity.id)
                .await?;
            let prompts = self
                .get_workflow_activity_prompts(&workflow_activity.id)
                .await?;
            let mut storage_systems = self
                .get_workflow_activity_storage_systems(&workflow_activity.id)
                .await?;
            if let Some(systems) = &request.storage_system_ids {
                for id in systems {
                    storage_systems.push(WorkflowActivityStorageSystem {
                        system_id: *id,
                        configuration: None,
                    })
                }
            }
            let activity = self
                .get_activity(&workflow_activity.activity_id)
                .await?
                .unwrap();
            let activity_inputs = self.get_activity_inputs(&activity.id).await?;
            let activity_outputs = self.get_activity_outputs(&activity.id).await?;
            let workflow_inputs = self
                .get_workflow_activity_inputs(&workflow_activity.id)
                .await?;
            let workflow_outputs = self
                .get_workflow_activity_outputs(&workflow_activity.id)
                .await?;
            let id = WorkflowJobId {
                queue: workflow_activity.queue.clone(),
                id: plan_id,
                index: jobs.len() as i32,
            };
            let job = WorkflowJob {
                parent: None,
                plan_id: WorkflowExecutionId {
                    id: plan_id,
                    queue: workflow.queue.to_owned(),
                },
                id: id.clone(),
                error: None,
                workflow_id: workflow.id.to_string(),
                metadata_id: request.metadata_id.map(|id| id.to_string()),
                metadata_version: request.metadata_version,
                profile_id: request.profile_id.map(|id| id.to_string()),
                workflow_activity,
                workflow_inputs,
                workflow_outputs,
                activity,
                activity_inputs,
                activity_outputs,
                context: None,
                supplementary_id: None,
                collection_id: request.collection_id.map(|id| id.to_string()),
                models,
                prompts,
                storage_systems,
                children: HashSet::new(),
                completed_children: HashSet::new(),
                failed_children: HashSet::new(),
                complete: false,
                finished: None,
                failures: 0,
            };
            if job.workflow_activity.execution_group == 1 {
                current_execution_group.push(id.index);
            }
            pending.insert(id.index);
            jobs.push(job);
        }
        Ok(WorkflowExecutionPlan {
            id: WorkflowExecutionId {
                queue: workflow.queue.to_owned(),
                id: plan_id,
            },
            delay_until: request.delay_until,
            enqueued: Utc::now(),
            error: None,
            active: HashSet::new(),
            complete: HashSet::new(),
            failed: HashSet::new(),
            cancelled: false,
            workflow: workflow.clone(),
            jobs,
            context: None,
            parent: None,
            supplementary_id: None,
            metadata_id: request.metadata_id,
            metadata_version: request.metadata_version,
            collection_id: request.collection_id,
            profile_id: request.profile_id,
            finished: None,
            max_failures: 10,
        })
    }

    #[tracing::instrument(skip(self, ctx, request))]
    pub async fn enqueue_workflow(
        &self,
        ctx: &BoscaContext,
        request: &mut EnqueueRequest,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        if (request.workflow_id.is_some() || request.workflow.is_some())
            && request.trait_id.is_some()
        {
            return Err(Error::new("cannot enqueue workflow and trait"));
        }
        if let Some(trait_id) = &request.trait_id {
            let mut plans = Vec::new();
            for workflow in ctx.workflow.get_workflows_by_trait(trait_id).await? {
                request.workflow = Some(workflow);
                let plan = self.get_new_execution_plan(request).await?;
                if request.wait_for_completion {
                    self.wait_for_execution_plan(&plan.id, request.delay_until)
                        .await?;
                }
                plans.push(plan);
            }
            Ok(plans)
        } else if request.workflow.is_some() {
            let mut plan = self.get_new_execution_plan(request).await?;
            if let Some(id) = self.queues.enqueue_plan(&mut plan).await? {
                if request.wait_for_completion {
                    self.wait_for_execution_plan(&id, request.delay_until)
                        .await?;
                }
            }
            Ok(vec![plan])
        } else if let Some(workflow_id) = &request.workflow_id {
            request.workflow = self.get_workflow(workflow_id).await?;
            let mut plan = self.get_new_execution_plan(request).await?;
            if let Some(id) = self.queues.enqueue_plan(&mut plan).await? {
                if request.wait_for_completion {
                    self.wait_for_execution_plan(&id, request.delay_until)
                        .await?;
                }
            }
            Ok(vec![plan])
        } else {
            Err(Error::new("must enqueue workflow or trait"))
        }
    }

    #[tracing::instrument(skip(self, workflow_id, metadata_id, metadata_version, collection_id))]
    pub async fn cancel_workflows(
        &self,
        workflow_id: &str,
        metadata_id: &Option<Uuid>,
        metadata_version: &Option<i32>,
        collection_id: &Option<Uuid>,
    ) -> Result<(), Error> {
        self.queues
            .cancel_workflows(workflow_id, metadata_id, metadata_version, collection_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id, delay_until))]
    async fn wait_for_execution_plan(
        &self,
        id: &WorkflowExecutionId,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<(), Error> {
        let can_wait = if let Some(delay_until) = delay_until {
            delay_until > Utc::now()
        } else {
            true
        };
        // TODO: use subscription
        if can_wait {
            loop {
                if let Some(plan) = self.queues.get_plan(id).await? {
                    if plan.complete.len() == plan.jobs.len() {
                        return Ok(());
                    }
                }
                info!("waiting for execution plan to complete");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        } else {
            warn!("not waiting for execution plan to complete, it has been delayed");
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, plan_id, context))]
    pub async fn set_execution_plan_context(
        &self,
        plan_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        self.queues
            .set_execution_plan_context(plan_id, context)
            .await
    }

    #[tracing::instrument(skip(self, job_id, context))]
    pub async fn set_execution_plan_job_context(
        &self,
        job_id: &WorkflowJobId,
        context: &Value,
    ) -> Result<(), Error> {
        self.queues
            .set_execution_plan_job_context(job_id, context)
            .await
    }

    #[tracing::instrument(skip(self, job_id, delayed_until))]
    pub async fn set_execution_plan_job_delayed(
        &self,
        job_id: &WorkflowJobId,
        delayed_until: DateTime<Utc>,
    ) -> Result<(), Error> {
        self.queues
            .set_execution_plan_job_delayed(job_id, delayed_until)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id))]
    pub async fn set_execution_plan_job_checkin(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        self.queues.set_execution_plan_job_checkin(job_id).await
    }

    #[tracing::instrument(skip(self, job_id))]
    pub async fn set_execution_plan_job_complete(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let plan = self.queues.set_execution_plan_job_complete(job_id).await?;
        if plan.finished.is_some() {
            self.notifier.workflow_plan_finished(&plan.id).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, job_id, error))]
    pub async fn set_execution_plan_job_failed(
        &self,
        job_id: &WorkflowJobId,
        error: &str,
    ) -> Result<(), Error> {
        let plan = self
            .queues
            .set_execution_plan_job_failed(job_id, error)
            .await?;
        if plan.finished.is_some() {
            self.notifier.workflow_plan_failed(&plan.id).await?;
        }
        Ok(())
    }

    /* queues */
}
