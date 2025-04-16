use crate::datastores::notifier::Notifier;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::collection::Collection;
use crate::models::security::principal::Principal;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use async_graphql::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::GenericClient;
use log::error;
use std::sync::Arc;
use uuid::Uuid;
use bosca_database::TracingPool;
use crate::context::BoscaContext;
use crate::workflow::core_workflow_ids::COLLECTION_PROCESS;

#[derive(Clone)]
pub struct CollectionWorkflowsDataStore {
    pool: TracingPool,
    notifier: Arc<Notifier>,
}

impl CollectionWorkflowsDataStore {
    pub fn new(pool: TracingPool, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_collection_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.collections.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
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

    #[tracing::instrument(skip(self, ctx, principal, collection, to_state_id, valid, status, success, complete))]
    #[allow(clippy::too_many_arguments)]
    pub async fn set_state(
        &self,
        ctx: &BoscaContext,
        principal: &Principal,
        collection: &Collection,
        to_state_id: &str,
        valid: Option<DateTime<Utc>>,
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
                .prepare("update collections set workflow_state_pending_id = null, workflow_state_valid = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&collection.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update collections set workflow_state_id = $1, workflow_state_valid = null, workflow_state_pending_id = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &collection.id]).await?;
        } else {
            let stmt = txn
                .prepare("update collections set workflow_state_pending_id = $1, workflow_state_valid = $2 where id = $3")
                .await?;
            txn.execute(&stmt, &[&state, &valid, &collection.id]).await?;
        }
        txn.commit().await?;
        self.on_collection_changed(ctx, &collection.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn set_ready(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set ready = now() where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, principal, collection, configurations))]
    pub async fn set_ready_and_enqueue(
        &self,
        ctx: &BoscaContext,
        principal: &Principal,
        collection: &Collection,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<(), Error> {
        if collection.ready.is_some() {
            return Err(Error::new("collection already ready"));
        }
        self.set_state(
            ctx,
            principal,
            collection,
            "draft",
            None,
            "move to draft during set to ready",
            true,
            false,
        )
        .await?;
        self.set_ready(ctx, &collection.id).await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(COLLECTION_PROCESS.to_string()),
            collection_id: Some(collection.id),
            configurations,
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(())
    }
}
