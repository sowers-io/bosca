use crate::context::BoscaContext;
use crate::datastores::content::tag::update_metadata_etag;
use crate::datastores::notifier::Notifier;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::METADATA_PROCESS;
use async_graphql::*;
use chrono::DateTime;
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

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.metadata.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
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

    #[tracing::instrument(skip(self, ctx, principal, metadata, to_state_id, valid, status, success, complete))]
    #[allow(clippy::too_many_arguments)]
    pub async fn set_state(
        &self,
        ctx: &BoscaContext,
        principal: &Principal,
        metadata: &Metadata,
        to_state_id: &str,
        valid: Option<DateTime<chrono::Utc>>,
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
                .prepare("update metadata set workflow_state_pending_id = null, workflow_state_valid = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&metadata.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update metadata set workflow_state_id = $1, workflow_state_pending_id = null, workflow_state_valid = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &metadata.id]).await?;
        } else {
            let stmt = txn
                .prepare("update metadata set workflow_state_pending_id = $1, workflow_state_valid = $2 where id = $3")
                .await?;
            txn.execute(&stmt, &[&state, &valid, &metadata.id]).await?;
        }
        txn.commit().await?;
        self.on_metadata_changed(ctx, &metadata.id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, version))]
    pub async fn validate(&self, ctx: &BoscaContext, id: &Uuid, version: i32) -> Result<(), Error> {
        self.validate_document(ctx, id, version).await?;
        self.validate_guide(ctx, id, version).await?;
        Ok(())
    }

    pub async fn validate_guide(&self, _: &BoscaContext, _: &Uuid, _: i32) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, version))]
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

    #[tracing::instrument(skip(self, ctx, id))]
    async fn set_metadata_ready(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata set ready = now(), modified = now() where id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        update_metadata_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata, configurations))]
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
        self.set_state(
            ctx,
            &ctx.principal,
            metadata,
            "draft",
            None,
            "move to draft during set to ready",
            true,
            false,
        )
        .await?;
        self.set_metadata_ready(ctx, &metadata.id).await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(METADATA_PROCESS.to_string()),
            metadata_id: Some(metadata.id),
            metadata_version: Some(metadata.version),
            configurations,
            ..Default::default()
        };
        workflow.enqueue_workflow(ctx, &mut request).await?;
        if metadata.content_type == "bosca/v-guide" {
            let steps = ctx
                .content
                .guides
                .get_guide_steps(&metadata.id, metadata.version, None, None)
                .await?;
            for step in steps {
                let metadata = ctx
                    .check_metadata_version_action(
                        &step.step_metadata_id,
                        step.step_metadata_version,
                        PermissionAction::View,
                    )
                    .await?;
                Box::pin(self.set_metadata_ready_and_enqueue(ctx, &metadata, None)).await?;
            }
        }
        Ok(true)
    }
}
