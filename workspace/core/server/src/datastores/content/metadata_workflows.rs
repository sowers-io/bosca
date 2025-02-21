use crate::context::BoscaContext;
use crate::datastores::notifier::Notifier;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::metadata::Metadata;
use crate::models::security::principal::Principal;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataWorkflowsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl MetadataWorkflowsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_metadata_plans(&self, id: &Uuid) -> Result<Vec<(Uuid, String)>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select plan_id, queue from metadata_workflow_plans where id = $1")
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

    pub async fn set_metadata_workflow_state(
        &self,
        principal: &Principal,
        metadata: &Metadata,
        to_state_id: &str,
        status: &str,
        success: bool,
        complete: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let state = to_state_id.to_owned();
        let status = status.to_owned();
        let stmt = txn.prepare_cached("insert into metadata_workflow_transition_history (metadata_id, from_state_id, to_state_id, principal, status, success, complete) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        txn.execute(
            &stmt,
            &[
                &metadata.id,
                &metadata.workflow_state_id,
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
                .prepare("update metadata set workflow_state_pending_id = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&metadata.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update metadata set workflow_state_id = $1, workflow_state_pending_id = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &metadata.id]).await?;
        } else {
            let stmt = txn
                .prepare("update metadata set workflow_state_pending_id = $1 where id = $2")
                .await?;
            txn.execute(&stmt, &[&state, &metadata.id]).await?;
        }
        txn.commit().await?;
        self.on_metadata_changed(&metadata.id).await?;
        Ok(())
    }

    pub async fn validate(&self, ctx: &BoscaContext, id: &Uuid, version: i32) -> Result<(), Error> {
        self.validate_document(ctx, id, version).await?;
        self.validate_guide(ctx, id, version).await?;
        Ok(())
    }

    pub async fn validate_guide(&self, _: &BoscaContext, _: &Uuid, _: i32) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    pub async fn validate_document(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        version: i32,
    ) -> Result<(), Error> {
        if let Some(document) = ctx.content.documents.get_document(id, version).await? {
            if let Some(template_id) = &document.template_metadata_id {
                if let Some(template_version) = &document.template_metadata_version {
                    if let Some(template) = ctx
                        .content
                        .documents
                        .get_template(template_id, *template_version)
                        .await?
                    {
                        if let Some(schema) = template.schema {
                            if !jsonschema::is_valid(&schema, &document.content) {
                                return Err(Error::new("document validation failed"));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn set_metadata_ready(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set ready = now() where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    pub async fn set_metadata_ready_and_enqueue(
        &self,
        ctx: &BoscaContext,
        metadata: &Metadata,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<bool, Error> {
        if metadata.ready.is_some() {
            return Ok(false);
        }

        self.validate(ctx, &metadata.id, metadata.version).await?;

        let workflow = &ctx.workflow;
        let process_id = "metadata.process".to_owned();
        self.set_metadata_workflow_state(
            &ctx.principal,
            metadata,
            "draft",
            "move to draft during set to ready",
            true,
            false,
        )
        .await?;
        self.set_metadata_ready(&metadata.id).await?;
        workflow
            .enqueue_metadata_workflow(
                &process_id,
                &metadata.id,
                &metadata.version,
                configurations.as_ref(),
                None,
            )
            .await?;
        self.on_metadata_changed(&metadata.id).await?;
        Ok(true)
    }
}
