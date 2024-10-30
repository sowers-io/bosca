use crate::models::workflow::activities::{
    Activity, ActivityInput, ActivityParameter, WorkflowActivity, WorkflowActivityInput,
    WorkflowActivityModel, WorkflowActivityParameter, WorkflowActivityPrompt,
    WorkflowActivityStorageSystem,
};
use crate::models::workflow::execution_plan::{
    WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJob, WorkflowJobId,
};
use crate::models::workflow::models::{Model, ModelInput};
use crate::models::workflow::prompts::{Prompt, PromptInput};
use crate::models::workflow::states::{WorkflowState, WorkflowStateInput};
use crate::models::workflow::storage_system_models::{StorageSystemModel, StorageSystemModelInput};
use crate::models::workflow::storage_systems::{StorageSystem, StorageSystemInput};
use crate::models::workflow::traits::TraitInput;
use crate::models::workflow::transitions::{Transition, TransitionInput};
use crate::models::workflow::workflows::{Workflow, WorkflowInput};
use crate::queue::message::MessageValue;
use crate::worklfow::job_queue::JobQueues;
use async_graphql::*;
use chrono::Utc;
use deadpool_postgres::{GenericClient, Pool};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;

#[derive(Clone)]
pub struct WorkflowDataStore {
    pool: Arc<Pool>,
    queues: JobQueues,
}

impl WorkflowDataStore {
    pub fn new(pool: Arc<Pool>, queues: JobQueues) -> Self {
        Self { pool, queues }
    }

    /* activities */

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
        Ok(())
    }

    pub async fn get_activities(&self) -> Result<Vec<Activity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activities")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(Activity::from).collect())
    }

    pub async fn get_activity(&self, id: &String) -> Result<Option<Activity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activities where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_activity_inputs(
        &self,
        activity_id: &String,
    ) -> Result<Vec<ActivityParameter>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activity_inputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(ActivityParameter::from).collect())
    }

    pub async fn get_activity_outputs(
        &self,
        activity_id: &String,
    ) -> Result<Vec<ActivityParameter>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from activity_outputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(ActivityParameter::from).collect())
    }

    /* activities */

    /* workflows */

    pub async fn get_workflows(&self) -> Result<Vec<Workflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from workflows").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Workflow::from).collect())
    }

    pub async fn get_workflows_by_trait(&self, trait_id: &String) -> Result<Vec<Workflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select w.* from workflows w inner join trait_workflows tw on (tw.workflow_id = w.id and tw.trait_id = $1)").await?;
        let rows = connection.query(&stmt, &[trait_id]).await?;
        Ok(rows.into_iter().map(Workflow::from).collect())
    }

    pub async fn add_workflow(&self, workflow: &WorkflowInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into workflows (id, name, description, queue, configuration) values ($1, $2, $3, $4, $5) returning id").await?;
        connection
            .query(
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
        Ok(())
    }

    pub async fn get_workflow(&self, id: &String) -> Result<Option<Workflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflows where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.remove(0).into()))
    }

    /* workflows */

    /* workflow activities */

    pub async fn add_workflow_activity(
        &self,
        activity: &WorkflowActivityInput,
    ) -> Result<i64, Error> {
        let connection = self.pool.get().await?;
        let execution_group = if activity.execution_group == 0 {
            1
        } else {
            activity.execution_group
        };
        let id: i64 = {
            let stmt = connection.prepare_cached("insert into workflow_activities (workflow_id, activity_id, execution_group, queue, configuration) values ($1, $2, $3, $4, $5) returning id").await?;
            let rows = connection
                .query(
                    &stmt,
                    &[
                        &activity.workflow_id,
                        &activity.activity_id,
                        &execution_group,
                        &activity.queue,
                        &activity.configuration,
                    ],
                )
                .await?;
            rows.first().unwrap().get(0)
        };
        {
            let stmt = connection.prepare_cached("insert into workflow_activity_inputs (activity_id, name, value) values ($1, $2, $3)").await?;
            for input in activity.inputs.iter() {
                connection
                    .execute(&stmt, &[&id, &input.name, &input.value])
                    .await?;
            }
        }
        {
            let stmt = connection.prepare_cached("insert into workflow_activity_outputs (activity_id, name, value) values ($1, $2, $3)").await?;
            for input in activity.outputs.iter() {
                connection
                    .execute(&stmt, &[&id, &input.name, &input.value])
                    .await?;
            }
        }
        {
            let stmt = connection.prepare_cached("insert into workflow_activity_models (activity_id, model_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.models.iter() {
                let mid = Uuid::parse_str(input.model_id.as_str())?;
                connection
                    .execute(&stmt, &[&id, &mid, &input.configuration])
                    .await?;
            }
        }
        {
            let stmt = connection.prepare_cached("insert into workflow_activity_prompts (activity_id, prompt_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.prompts.iter() {
                let pid = Uuid::parse_str(input.prompt_id.as_str())?;
                connection
                    .execute(&stmt, &[&id, &pid, &input.configuration])
                    .await?;
            }
        }
        {
            let stmt = connection.prepare_cached("insert into workflow_activity_storage_systems (activity_id, storage_system_id, configuration) values ($1, $2, $3)").await?;
            for input in activity.storage_systems.iter() {
                let sid = Uuid::parse_str(input.system_id.as_str())?;
                connection
                    .execute(&stmt, &[&id, &sid, &input.configuration])
                    .await?;
            }
        }
        Ok(id)
    }

    pub async fn get_workflow_activities(
        &self,
        workflow_id: &String,
    ) -> Result<Vec<WorkflowActivity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activities where workflow_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[workflow_id]).await?;
        Ok(rows.iter().map(WorkflowActivity::from).collect())
    }

    pub async fn get_workflow_activity_inputs(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityParameter>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_inputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(WorkflowActivityParameter::from).collect())
    }

    pub async fn get_workflow_activity_outputs(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityParameter>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_outputs where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(WorkflowActivityParameter::from).collect())
    }

    /* workflow activities */

    /* workflow activity models */

    pub async fn get_workflow_activity_models(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityModel>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_models where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(WorkflowActivityModel::from).collect())
    }

    /* workflow activity models */

    /* workflow activity prompts */

    pub async fn get_workflow_activity_prompts(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityPrompt>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activity_prompts where activity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.iter().map(WorkflowActivityPrompt::from).collect())
    }

    /* workflow activity prompts */

    /* workflow activity storage systems */

    pub async fn get_workflow_activity_storage_systems(
        &self,
        activity_id: &i64,
    ) -> Result<Vec<WorkflowActivityStorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from workflow_activity_storage_systems where activity_id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows
            .iter()
            .map(WorkflowActivityStorageSystem::from)
            .collect())
    }

    /* workflow activity storage systems */

    /* models */

    pub async fn get_models(&self) -> Result<Vec<Model>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from models").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Model::from).collect())
    }

    pub async fn get_model(&self, id: &Uuid) -> Result<Option<Model>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from models where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.remove(0).into()))
    }

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
        Ok(rows.first().unwrap().get(0))
    }

    /* models */

    /* states */

    pub async fn get_states(&self) -> Result<Vec<WorkflowState>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_states")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(WorkflowState::from).collect())
    }

    pub async fn get_state(&self, id: &str) -> Result<Option<WorkflowState>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_states where id = $1")
            .await?;
        let id = id.to_owned();
        let mut rows = connection.query(&stmt, &[&id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.remove(0).into()))
    }

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
        Ok(())
    }

    /* states */

    /* storage systems */

    pub async fn get_storage_systems(&self) -> Result<Vec<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(StorageSystem::from).collect())
    }

    pub async fn get_default_search_storage_system(&self) -> Result<StorageSystem, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems where name = 'Default Search'")
            .await?;
        let mut rows = connection.query(&stmt, &[]).await?;
        Ok(rows.remove(0).into())
    }

    pub async fn get_storage_system(&self, id: &Uuid) -> Result<Option<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems where id = $1")
            .await?;
        let mut rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.remove(0).into()))
    }

    pub async fn add_storage_system(&self, system: &StorageSystemInput) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into storage_systems (type, name, description, configuration) values ($1, $2, $3, $4) returning id").await?;
        let rows = connection
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
        // TODO: initialize storage system
        Ok(rows.first().unwrap().get(0))
    }

    /* storage systems */

    /* storage system models */

    pub async fn add_storage_system_model(
        &self,
        system_id: &Uuid,
        model: &StorageSystemModelInput,
    ) -> Result<(), Error> {
        let model_id = Uuid::parse_str(model.model_id.as_str())?;
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into storage_system_models (system_id, model_id, configuration) values ($1, $2, $3)").await?;
        connection
            .query(&stmt, &[&system_id, &model_id, &model.configuration])
            .await?;
        Ok(())
    }

    pub async fn get_storage_system_models(
        &self,
        id: &Uuid,
    ) -> Result<Vec<StorageSystemModel>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_system_models where system_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.into_iter().map(StorageSystemModel::from).collect())
    }

    /* storage system models */

    /* prompts */

    pub async fn add_prompt(&self, prompt: &PromptInput) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into prompts (name, description, system_prompt, user_prompt, input_type, output_type) values ($1, $2, $3, $4, $5, $6) returning id").await?;
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
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(Uuid::nil());
        }
        Ok(rows.first().unwrap().get(0))
    }

    pub async fn get_prompts(&self) -> Result<Vec<Prompt>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from prompts").await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(Prompt::from).collect())
    }

    pub async fn get_prompt(&self, id: &Uuid) -> Result<Option<Prompt>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from prompts where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    /* prompts */

    /* traits */

    pub async fn add_trait(&self, t: &TraitInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into traits (id, name, description) values ($1, $2, $3)")
            .await?;
        connection
            .query(&stmt, &[&t.id, &t.name, &t.description])
            .await?;
        drop(stmt);
        let stmt = connection
            .prepare_cached("insert into trait_workflows (trait_id, workflow_id) values ($1, $2)")
            .await?;
        for workflow_id in t.workflow_ids.iter() {
            connection.execute(&stmt, &[&t.id, workflow_id]).await?;
        }
        Ok(())
    }

    /* traits */

    /* transitions */

    pub async fn add_transition(&self, t: &TransitionInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into workflow_state_transitions (from_state_id, to_state_id, description) values ($1, $2, $3)").await?;
        connection
            .query(&stmt, &[&t.from_state_id, &t.to_state_id, &t.description])
            .await?;
        Ok(())
    }

    pub async fn get_transitions(&self) -> Result<Vec<Transition>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_state_transitions")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Transition::from).collect())
    }

    pub async fn get_transition(&self, from_state_id: &str, to_state_id: &str) -> Result<Option<Transition>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_state_transitions where from_state_id = $1 and to_state_id = $2")
            .await?;
        let from = from_state_id.to_owned();
        let to = to_state_id.to_owned();
        let rows = connection.query(&stmt, &[&from, &to]).await?;
        Ok(rows.into_iter().next().map(Transition::from))
    }

    /* transitions */

    /* queues */

    pub async fn create_queues(&self) -> Result<(), Error> {
        self.queues.initialize().await?;
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select distinct queue from (select queue from workflows union select queue from workflow_activities)").await?;
        let rows = connection.query(&stmt, &[]).await?;
        for row in rows.iter() {
            let queue: String = row.get("queue");
            self.queues.create_queue(&queue).await?;
        }
        Ok(())
    }

    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowExecutionId,
        workflow_ids: &Vec<String>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut plans = Vec::<WorkflowExecutionPlan>::new();
        let job = self.queues.get(job_id, false).await?;
        if job.is_none() {
            return Err(Error::new("missing job"));
        }
        let job = match job.unwrap() {
            MessageValue::Job(job) => job,
            _ => return Err(Error::new("unexpected job type")),
        };
        let metadata_id = job
            .metadata_id
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let collection_id = job
            .collection_id
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        for workflow_id in workflow_ids {
            let workflow = self.get_workflow(workflow_id).await?;
            if workflow.is_none() {
                return Err(Error::new("workflow not found"));
            }
            let workflow = workflow.unwrap();
            let plan = self
                .get_new_execution_plan(&workflow, collection_id, metadata_id, job.version, None)
                .await?;
            plans.push(plan);
        }
        self.queues
            .enqueue_job_child_workflows(job_id, &plans)
            .await
    }

    pub async fn enqueue_execution_job(
        &self,
        plan_id: &WorkflowExecutionId,
        job_index: i32,
    ) -> Result<Option<WorkflowExecutionId>, Error> {
        self.queues.enqueue_execution_job(plan_id, job_index).await
    }

    pub async fn dequeue_next_execution(
        &self,
        queue: &String,
    ) -> Result<Option<MessageValue>, Error> {
        self.queues.dequeue(queue).await
    }

    pub async fn get_execution_plan(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        let plan = self.queues.get(id, false).await?;
        if plan.is_none() {
            let plan = self.queues.get(id, true).await?;
            if plan.is_none() {
                return Ok(None);
            }
            match plan.unwrap() {
                MessageValue::Plan(plan) => Ok(Some(plan)),
                _ => Ok(None),
            }
        } else {
            match plan.unwrap() {
                MessageValue::Plan(plan) => Ok(Some(plan)),
                _ => Ok(None),
            }
        }
    }

    pub async fn get_execution_plans(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
        archived: bool,
    ) -> Result<Vec<MessageValue>, Error> {
        self.queues.get_all(queue, offset, limit, archived).await
    }

    pub async fn get_new_execution_plan(
        &self,
        workflow: &Workflow,
        collection_id: Option<Uuid>,
        metadata_id: Option<Uuid>,
        version: Option<i32>,
        configuration: Option<&Vec<WorkflowConfigurationInput>>
    ) -> Result<WorkflowExecutionPlan, Error> {
        let mut jobs = Vec::<WorkflowJob>::new();
        let activities = self.get_workflow_activities(&workflow.id).await?;
        let mut pending = HashSet::<WorkflowJobId>::new();
        let mut current = Vec::<WorkflowJobId>::new();
        let mut configuration_overrides = HashMap::new();
        if let Some(overrides) = configuration {
            for o in overrides.iter() {
                configuration_overrides.insert(o.activity_id.to_owned(), o.configuration.to_owned());
            }
        }
        for mut workflow_activity in activities.into_iter() {
            if let Some(Value::Object(o)) = configuration_overrides.get(&workflow_activity.activity_id) {
                if let Value::Object(ref mut o2) = workflow_activity.configuration {
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
            let storage_systems = self
                .get_workflow_activity_storage_systems(&workflow_activity.id)
                .await?;
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
                id: 0,
                index: jobs.len() as i32,
            };
            let job = WorkflowJob {
                id: id.clone(),
                execution_plan: WorkflowExecutionId {
                    queue: workflow.queue.clone(),
                    id: 0,
                },
                error: None,
                workflow_id: workflow.id.to_string(),
                metadata_id: metadata_id.map(|id| id.to_string()),
                version,
                workflow_activity,
                workflow_inputs,
                workflow_outputs,
                activity,
                activity_inputs,
                activity_outputs,
                context: Value::Null,
                supplementary_id: None,
                collection_id: collection_id.map(|id| id.to_string()),
                models,
                prompts,
                storage_systems,
                children: HashSet::new(),
                completed_children: HashSet::new(),
                failed_children: HashSet::new(),
                complete: false,
            };
            if job.workflow_activity.execution_group == 1 {
                current.push(id.clone());
            }
            pending.insert(id.clone());
            jobs.push(job);
        }
        Ok(WorkflowExecutionPlan {
            plan_id: 0,
            enqueued: Utc::now(),
            error: None,
            pending,
            current,
            running: HashSet::new(),
            complete: HashSet::new(),
            failed: HashSet::new(),
            metadata_id: metadata_id.map(|id| id.to_string()),
            workflow: workflow.clone(),
            jobs,
            context: Value::Null,
            parent: None,
            next: None,
            supplementary_id: None,
            version,
            collection_id: None,
        })
    }

    pub async fn enqueue_metadata_workflow(
        &self,
        workflow_id: &String,
        metadata_id: &Uuid,
        version: &i32,
        configurations: Option<&Vec<WorkflowConfigurationInput>>,
        wait_for_completion: Option<bool>,
    ) -> Result<WorkflowExecutionPlan, Error> {
        if let Some(workflow) = self.get_workflow(workflow_id).await? {
            let mut plan = self
                .get_new_execution_plan(&workflow, None, Some(*metadata_id), Some(*version), configurations)
                .await?;
            let value = MessageValue::Plan(plan.clone());
            let id = self.queues.enqueue(&plan.workflow.queue, &value).await?;
            plan.plan_id = id;
            if wait_for_completion.is_some() && wait_for_completion.unwrap() {
                // TODO: use subscription
                loop {
                    let exists = self.queues.exists(&plan.workflow.queue, id).await?;
                    if !exists {
                        break;
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            Ok(plan)
        } else {
            Err(Error::new("workflow not found"))
        }
    }

    pub async fn enqueue_metadata_trait_workflow(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        trait_id: &String,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        let mut plans = Vec::<WorkflowExecutionPlan>::new();
        for workflow in self.get_workflows_by_trait(trait_id).await? {
            let mut plan = self
                .get_new_execution_plan(&workflow, None, Some(*metadata_id), Some(*version), None)
                .await?;
            let value = MessageValue::Plan(plan.clone());
            let id = self.queues.enqueue(&workflow.queue, &value).await?;
            plan.plan_id = id;
            plans.push(plan);
        }
        Ok(plans)
    }

    pub async fn enqueue_collection_workflow(
        &self,
        workflow_id: &String,
        collection_id: &Uuid,
        configurations: Option<&Vec<WorkflowConfigurationInput>>,
        wait_for_completion: Option<bool>,
    ) -> Result<WorkflowExecutionPlan, Error> {
        if let Some(workflow) = self.get_workflow(workflow_id).await? {
            let mut plan = self
                .get_new_execution_plan(&workflow, Some(*collection_id), None, None, configurations)
                .await?;
            let value = MessageValue::Plan(plan.clone());
            let id = self.queues.enqueue(&plan.workflow.queue, &value).await?;
            plan.plan_id = id;
            if wait_for_completion.is_some() && wait_for_completion.unwrap() {
                // TODO: use subscription
                loop {
                    let exists = self.queues.exists(&plan.workflow.queue, id).await?;
                    if !exists {
                        break;
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            Ok(plan)
        } else {
            Err(Error::new("workflow not found"))
        }
    }

    pub async fn set_execution_plan_context(
        &self,
        plan_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        self.queues
            .set_execution_plan_context(plan_id, context)
            .await
    }

    pub async fn set_execution_job_context(
        &self,
        job_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        self.queues.set_execution_job_context(job_id, context).await
    }

    pub async fn set_execution_plan_job_checkin(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        self.queues.set_execution_plan_job_checkin(job_id).await
    }

    pub async fn set_execution_plan_job_complete(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        self.queues.set_execution_plan_job_complete(job_id).await
    }

    pub async fn set_execution_plan_job_failed(
        &self,
        job_id: &WorkflowJobId,
        error: &str,
    ) -> Result<(), Error> {
        self.queues
            .set_execution_plan_job_failed(job_id, error)
            .await
    }

    /* queues */
}
