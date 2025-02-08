use crate::datastores::notifier::Notifier;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
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
use crate::models::workflow::traits::{Trait, TraitInput};
use crate::models::workflow::transitions::{Transition, TransitionInput};
use crate::models::workflow::workflows::{Workflow, WorkflowInput};
use crate::worklfow::queue::JobQueues;
use async_graphql::*;
use chrono::Utc;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use meilisearch_sdk::client::Client;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkflowDataStore {
    pool: Arc<Pool>,
    queues: JobQueues,
    notifier: Arc<Notifier>,
    search: Arc<Client>,
}

impl WorkflowDataStore {
    pub fn new(
        pool: Arc<Pool>,
        queues: JobQueues,
        notifier: Arc<Notifier>,
        search: Arc<Client>,
    ) -> Self {
        Self {
            pool,
            queues,
            notifier,
            search,
        }
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
        self.notifier.activity_changed(&activity.id).await?;
        Ok(())
    }

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

    pub async fn delete_activity(&self, activity_id: &str) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        connection
            .execute("delete from activities where id = $1", &[&activity_id])
            .await?;
        self.notifier.activity_changed(activity_id).await?;
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
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        self.add_workflow_txn(&txn, workflow).await?;
        txn.commit().await?;
        self.notifier.workflow_changed(&workflow.id).await?;
        Ok(())
    }

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
        for activity in &workflow.activities {
            self.add_workflow_activity(&txn, &workflow.id, activity)
                .await?;
        }
        txn.commit().await?;
        self.notifier.workflow_changed(&workflow.id).await?;
        Ok(())
    }

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

    pub async fn delete_workflow(&self, id: &str) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let id = id.to_owned();
        self.delete_workflow_txn(&txn, &id, true).await?;
        txn.commit().await?;
        self.notifier.workflow_changed(&id).await?;
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
        Ok(id)
    }

    pub async fn get_workflow_activity(
        &self,
        activity_id: &i64,
    ) -> Result<Option<WorkflowActivity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activities where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[activity_id]).await?;
        Ok(rows.first().map(WorkflowActivity::from))
    }

    pub async fn get_workflow_activities(
        &self,
        workflow_id: &String,
    ) -> Result<Vec<WorkflowActivity>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_activities where workflow_id = $1 order by execution_group asc")
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
        let id: Uuid = rows.first().unwrap().get(0);
        let id_str = id.to_string();
        self.notifier.model_changed(&id_str).await?;
        Ok(id)
    }

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
        self.notifier.model_changed(&id_str).await?;
        Ok(())
    }

    pub async fn delete_model(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from models where id = $5")
            .await?;
        connection.execute(&stmt, &[&id]).await?;
        let id_str = id.to_string();
        self.notifier.model_changed(&id_str).await?;
        Ok(())
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
        self.notifier.state_changed(&state.id).await?;
        Ok(())
    }

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
        self.notifier.state_changed(&state.id).await?;
        Ok(())
    }

    pub async fn delete_state(&self, id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from workflow_states where id = $1")
            .await?;
        connection.execute(&stmt, &[&id]).await?;
        self.notifier.state_changed(id).await?;
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
        Ok(rows.iter().map(StorageSystem::from).collect())
    }

    pub async fn get_default_search_storage_system(&self) -> Result<Option<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems where name = 'Default Search'")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_storage_system(&self, id: &Uuid) -> Result<Option<StorageSystem>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from storage_systems where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn add_storage_system(&self, system: &StorageSystemInput) -> Result<Uuid, Error> {
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

        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;
        // TODO: initialize storage system
        Ok(id)
    }

    pub async fn initialize_default_search_index(&self) -> Result<(), Error> {
        let systems = self.get_storage_systems().await?;
        let Some(storage_system) = systems.iter().find(|s| s.name == "Default Search") else {
            return Err(Error::new("Default search storage system not found".to_string()));
        };
        let Some(configuration) = &storage_system.configuration else {
            return Ok(());
        };
        let index_name = configuration
            .get("indexName")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
        let create_task = self
            .search
            .create_index(index_name.clone(), Some("_id"))
            .await?;
        self.search.wait_for_task(create_task, None, None).await?;
        let index = self.search.get_index(index_name).await?;
        let mut settings = index.get_settings().await?;
        settings.filterable_attributes = Some(vec!["_type".to_owned()]);
        index.set_settings(&settings).await?;
        Ok(())
    }

    pub async fn edit_storage_system(
        &self,
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
        // TODO: initialize storage system
        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;
        Ok(())
    }

    /* storage systems */

    /* storage system models */

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

    pub async fn delete_storage_system(&self, id: &Uuid) -> Result<bool, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from storage_system where idf = $1")
            .await?;
        connection.query(&stmt, &[id]).await?;
        let id_str = id.to_string();
        self.notifier.storage_system_changed(&id_str).await?;
        Ok(true)
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
        let id: Uuid = rows.first().unwrap().get(0);
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(id)
    }

    pub async fn edit_prompt(&self, id: &Uuid, prompt: &PromptInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update prompts set name = $1, description = $2, system_prompt = $3, user_prompt = $4, input_type = $5, output_type = $6 where id = $7").await?;
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
                    id,
                ],
            )
            .await?;
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(())
    }

    pub async fn delete_prompt(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from prompts where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        let id_str = id.to_string();
        self.notifier.prompt_changed(&id_str).await?;
        Ok(())
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

    pub async fn get_trait(&self, id: &String) -> Result<Option<Trait>, Error> {
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
            return Ok(Some(t));
        }
        Ok(None)
    }

    pub async fn add_trait(&self, t: &TraitInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into traits (id, name, description, delete_workflow_id) values ($1, $2, $3, $4)")
            .await?;
        connection
            .query(&stmt, &[&t.id, &t.name, &t.description, &t.delete_workflow_id])
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

    pub async fn edit_trait(&self, t: &TraitInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let stmt = transaction
            .prepare_cached("update traits set name = $2, description = $3, delete_workflow_id = $4 where id = $1")
            .await?;
        transaction
            .query(&stmt, &[&t.id, &t.name, &t.description, &t.delete_workflow_id])
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
        self.notifier.trait_changed(&t.id).await?;
        Ok(())
    }

    pub async fn delete_trait(&self, id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        connection
            .execute("delete from traits where id = $1", &[&id])
            .await?;
        self.notifier.trait_changed(id).await?;
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

    pub async fn edit_transition(&self, t: &TransitionInput) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update workflow_state_transitions set description = $3 where from_state_id = $1 and to_state_id = $2").await?;
        connection
            .query(&stmt, &[&t.from_state_id, &t.to_state_id, &t.description])
            .await?;
        Ok(())
    }

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

    pub async fn get_transition(
        &self,
        from_state_id: &str,
        to_state_id: &str,
    ) -> Result<Option<Transition>, Error> {
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

    pub async fn expire_all(&self) -> Result<(), Error> {
        self.queues.check_for_expiration(i64::MAX).await?;
        Ok(())
    }

    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowJobId,
        workflow_ids: &Vec<String>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut plans = Vec::<WorkflowExecutionPlan>::new();
        let Some(plan) = self.queues.get_plan_by_job(job_id).await? else {
            return Err(Error::new("missing plan"));
        };
        let job = plan.jobs.get(job_id.index as usize).unwrap();
        for workflow_id in workflow_ids {
            let workflow = self.get_workflow(workflow_id).await?;
            if workflow.is_none() {
                return Err(Error::new("workflow not found"));
            }
            let workflow = workflow.unwrap();
            let plan = self
                .get_new_execution_plan(
                    &workflow,
                    job.collection_id
                        .as_ref()
                        .map(|id| Uuid::parse_str(id).unwrap()),
                    job.metadata_id
                        .as_ref()
                        .map(|id| Uuid::parse_str(id).unwrap()),
                    job.metadata_version,
                    None,
                )
                .await?;
            plans.push(plan);
        }
        self.queues
            .enqueue_job_child_workflows(job_id, &plans)
            .await
    }

    pub async fn enqueue_job_child_workflow(
        &self,
        job_id: &WorkflowJobId,
        workflow_id: &str,
        configurations: Option<&Vec<WorkflowConfigurationInput>>,
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
        let workflow = self.get_workflow(&workflow_id).await?;
        if workflow.is_none() {
            return Err(Error::new("workflow not found"));
        }
        let workflow = workflow.unwrap();
        let plan = self
            .get_new_execution_plan(
                &workflow,
                collection_id,
                metadata_id,
                job.metadata_version,
                configurations,
            )
            .await?;
        plans.push(plan);
        Ok(self
            .queues
            .enqueue_job_child_workflows(job_id, &plans)
            .await?
            .first()
            .unwrap()
            .clone())
    }

    pub async fn dequeue_next_execution(
        &self,
        queue: &str,
    ) -> Result<Option<WorkflowJob>, Error> {
        self.queues.dequeue(queue).await
    }

    pub async fn get_execution_plan(
        &self,
        id: &WorkflowExecutionId,
    ) -> Result<Option<WorkflowExecutionPlan>, Error> {
        self.queues.get_plan(id).await
    }

    pub async fn get_execution_plans(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WorkflowExecutionPlan>, Error> {
        self.queues.get_all_plans(queue, offset, limit).await
    }

    pub async fn get_new_execution_plan(
        &self,
        workflow: &Workflow,
        collection_id: Option<Uuid>,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        configurations: Option<&Vec<WorkflowConfigurationInput>>,
    ) -> Result<WorkflowExecutionPlan, Error> {
        let mut jobs = Vec::<WorkflowJob>::new();
        let activities = self.get_workflow_activities(&workflow.id).await?;
        let mut pending = HashSet::<i32>::new();
        let mut current_execution_group = Vec::<i32>::new();
        let mut configuration_overrides = HashMap::new();
        if let Some(overrides) = configurations {
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
                if workflow_activity.configuration.is_null() {
                    workflow_activity.configuration = Value::Object(o.clone());
                } else if let Value::Object(ref mut o2) = workflow_activity.configuration {
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
                id: plan_id,
                index: jobs.len() as i32,
            };
            let job = WorkflowJob {
                plan_id: WorkflowExecutionId {
                    id: plan_id,
                    queue: workflow.queue.to_owned(),
                },
                id: id.clone(),
                error: None,
                workflow_id: workflow.id.to_string(),
                metadata_id: metadata_id.map(|id| id.to_string()),
                metadata_version,
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
                finished: None,
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
            enqueued: Utc::now(),
            error: None,
            pending,
            current_execution_group,
            running: HashSet::new(),
            complete: HashSet::new(),
            failed: HashSet::new(),
            metadata_id,
            workflow: workflow.clone(),
            jobs,
            context: Value::Null,
            parent: None,
            next: None,
            supplementary_id: None,
            metadata_version,
            collection_id,
            finished: None,
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
                .get_new_execution_plan(
                    &workflow,
                    None,
                    Some(*metadata_id),
                    Some(*version),
                    configurations,
                )
                .await?;
            let id = self.queues.enqueue_plan(&mut plan).await?;
            if wait_for_completion.is_some() && wait_for_completion.unwrap() {
                // TODO: use subscription
                loop {
                    if let Some(plan) = self.queues.get_plan(&id).await? {
                        if plan.complete.len() == plan.jobs.len() {
                            break;
                        }
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
            self.queues.enqueue_plan(&mut plan).await?;
            plans.push(plan);
        }
        Ok(plans)
    }

    pub async fn enqueue_collection_workflow(
        &self,
        workflow_id: &str,
        collection_id: &Uuid,
        configurations: Option<&Vec<WorkflowConfigurationInput>>,
        wait_for_completion: Option<bool>,
    ) -> Result<WorkflowExecutionPlan, Error> {
        if let Some(workflow) = self.get_workflow(&workflow_id.to_string()).await? {
            let mut plan = self
                .get_new_execution_plan(&workflow, Some(*collection_id), None, None, configurations)
                .await?;
            let id = self.queues.enqueue_plan(&mut plan).await?;
            if wait_for_completion.is_some() && wait_for_completion.unwrap() {
                // TODO: use subscription
                loop {
                    if let Some(plan) = self.queues.get_plan(&id).await? {
                        if plan.complete.len() == plan.jobs.len() {
                            break;
                        }
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
        job_id: &WorkflowJobId,
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
