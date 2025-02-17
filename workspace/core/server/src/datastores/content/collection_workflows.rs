use crate::datastores::notifier::Notifier;
use crate::datastores::workflow::WorkflowDataStore;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::collection::Collection;
use crate::models::security::principal::Principal;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionWorkflowsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl CollectionWorkflowsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_collection_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_plans(&self, id: &Uuid) -> Result<Vec<(Uuid, String)>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select plan_id, queue from collection_workflow_plans where id = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        Ok(results
            .iter()
            .map(|r| {
                let plan_id: Uuid = r.get("plan_id");
                let queue: String = r.get("queue");
                (plan_id, queue)
            })
            .collect())
    }

    pub async fn set_state(
        &self,
        principal: &Principal,
        collection: &Collection,
        to_state_id: &str,
        status: &str,
        success: bool,
        complete: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let state = to_state_id.to_owned();
        let status = status.to_owned();
        let stmt = txn.prepare_cached("insert into collection_workflow_transition_history (collection_id, from_state_id, to_state_id, principal, status, success, complete) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        txn.execute(
            &stmt,
            &[
                &collection.id,
                &collection.workflow_state_id,
                &state,
                &principal.id,
                &status,
                &success,
                &complete,
            ],
        )
        .await?;
        if !success {
            let stmt = txn
                .prepare("update collections set workflow_state_pending_id = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&collection.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update collections set workflow_state_id = $1, workflow_state_pending_id = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &collection.id]).await?;
        } else {
            let stmt = txn
                .prepare("update collections set workflow_state_pending_id = $1 where id = $2")
                .await?;
            txn.execute(&stmt, &[&state, &collection.id]).await?;
        }
        txn.commit().await?;
        self.on_collection_changed(&collection.id).await?;
        Ok(())
    }

    pub async fn set_ready(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set ready = now() where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_collection_changed(id).await?;
        Ok(())
    }

    pub async fn set_ready_and_enqueue(
        &self,
        workflow: &WorkflowDataStore,
        principal: &Principal,
        collection: &Collection,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<(), Error> {
        if collection.ready.is_some() {
            return Err(Error::new("collection already ready"));
        }
        let process_id = "collection.process".to_owned();
        self.set_state(
            principal,
            collection,
            "draft",
            "move to draft during set to ready",
            true,
            false,
        )
        .await?;
        workflow
            .enqueue_collection_workflow(&process_id, &collection.id, configurations.as_ref(), None)
            .await?;
        self.set_ready(&collection.id).await?;
        self.on_collection_changed(&collection.id).await?;
        Ok(())
    }
}
