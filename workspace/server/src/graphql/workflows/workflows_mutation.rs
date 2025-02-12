use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::graphql::workflows::models_mutation::ModelsMutationObject;
use crate::graphql::workflows::states_mutation::WorkflowStatesMutationObject;
use crate::graphql::workflows::prompts_mutation::PromptsMutationObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::models::workflow::execution_plan::{WorkflowExecutionIdInput, WorkflowJobIdInput};
use crate::models::workflow::workflows::WorkflowInput;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::content::content::FindAttributeInput;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::graphql::workflows::activities_mutation::ActivitiesMutationObject;
use crate::graphql::workflows::storage_systems_mutation::StorageSystemsMutationObject;
use crate::graphql::workflows::traits_mutation::TraitsMutationObject;
use crate::graphql::workflows::transitions_mutation::TransitionsMutationObject;
use crate::models::workflow::transitions::BeginTransitionInput;
use crate::util::transition::begin_transition;

pub(crate) struct WorkflowsMutationObject {}

#[Object(name = "WorkflowsMutation")]
impl WorkflowsMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        workflow: WorkflowInput,
    ) -> Result<WorkflowObject, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.add_workflow(&workflow).await?;
        if let Some(workflow) = ctx.workflow
            .get_workflow(&workflow.id)
            .await?
            .map(WorkflowObject::new)
        {
            return Ok(workflow);
        }
        Err(Error::new("missing workflow"))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        workflow: WorkflowInput,
    ) -> Result<WorkflowObject, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_workflow(&workflow).await?;
        if let Some(workflow) = ctx.workflow
            .get_workflow(&workflow.id)
            .await?
            .map(WorkflowObject::new)
        {
            return Ok(workflow);
        }
        Err(Error::new("missing workflow"))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.delete_workflow(&id).await?;
        Ok(true)
    }

    async fn models(&self) -> ModelsMutationObject {
        ModelsMutationObject {}
    }

    async fn states(&self) -> WorkflowStatesMutationObject {
        WorkflowStatesMutationObject {}
    }

    async fn transitions(&self) -> TransitionsMutationObject {
        TransitionsMutationObject {}
    }

    async fn traits(&self) -> TraitsMutationObject {
        TraitsMutationObject {}
    }

    async fn activities(&self) -> ActivitiesMutationObject {
        ActivitiesMutationObject {}
    }

    async fn prompts(&self) -> PromptsMutationObject {
        PromptsMutationObject {}
    }

    async fn storage_systems(&self) -> StorageSystemsMutationObject { StorageSystemsMutationObject {} }

    async fn expire_all(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow.expire_all().await?;
        Ok(true)
    }

    async fn begin_transition(
        &self,
        ctx: &Context<'_>,
        request: BeginTransitionInput,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        begin_transition(ctx, &request, configurations.as_ref()).await?;
        Ok(true)
    }

    async fn enqueue_child_workflows(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        workflow_ids: Vec<String>,
    ) -> Result<Vec<WorkflowExecutionIdObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(ctx.workflow
            .enqueue_job_child_workflows(&job_id.into(), &workflow_ids)
            .await?
            .into_iter()
            .map(WorkflowExecutionIdObject::new)
            .collect())
    }

    async fn enqueue_child_workflow(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        workflow_id: String,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<WorkflowExecutionIdObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(WorkflowExecutionIdObject::new(ctx.workflow.enqueue_job_child_workflow(&job_id.into(), &workflow_id, configurations.as_ref()).await?))
    }

    async fn find_and_enqueue_workflow(
        &self,
        ctx: &Context<'_>,
        workflow_id: String,
        attributes: Vec<FindAttributeInput>,
        content_types: Option<Vec<String>>,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<Vec<WorkflowExecutionIdObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let mut ids = Vec::new();
        // TODO: page through items
        for metadata in ctx.content.metadata.find(&attributes, &content_types, None, None, 0, 10000).await? {
            let id = ctx.workflow
                .enqueue_metadata_workflow(&workflow_id, &metadata.id, &metadata.version, configurations.as_ref(), None)
                .await?;
            ids.push(id);
        }
        for collection in ctx.content.collections.find(&attributes, None,0, 1000).await? {
            let id = ctx.workflow
                .enqueue_collection_workflow(&workflow_id, &collection.id, configurations.as_ref(), None)
                .await?;
            ids.push(id);
        }
        Ok(ids.into_iter().map(|plan| WorkflowExecutionIdObject::new(plan.id.clone())).collect())
    }

    async fn enqueue_workflow(
        &self,
        ctx: &Context<'_>,
        workflow_id: String,
        collection_id: Option<String>,
        metadata_id: Option<String>,
        version: Option<i32>,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<WorkflowExecutionIdObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let workflow = if let Some(metadata_id) = metadata_id {
            let id = Uuid::parse_str(metadata_id.as_str())?;
            if version.is_none() {
                return Err(Error::new("a version is required"));
            }
            
            ctx.workflow
                .enqueue_metadata_workflow(&workflow_id, &id, version.as_ref().unwrap(), configurations.as_ref(), None)
                .await?
        } else if let Some(collection_id) = collection_id {
            let id = Uuid::parse_str(collection_id.as_str())?;
            
            ctx.workflow
                .enqueue_collection_workflow(&workflow_id, &id, configurations.as_ref(), None)
                .await?
        } else {
            return Err(Error::new("you must provide either a collection_id or a metadata_id"));
        };
        Ok(WorkflowExecutionIdObject::new(workflow.id.clone()))
    }

    async fn set_execution_plan_context(
        &self,
        ctx: &Context<'_>,
        plan_id: WorkflowExecutionIdInput,
        context: Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_context(&plan_id.into(), &context)
            .await?;
        Ok(true)
    }

    async fn set_execution_job_context(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        context: Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_job_context(&job_id.into(), &context)
            .await?;
        Ok(true)
    }

    async fn set_execution_plan_job_checkin(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_job_checkin(&job_id.into())
            .await?;
        Ok(true)
    }

    async fn set_execution_plan_job_complete(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_job_complete(&job_id.into())
            .await?;
        Ok(true)
    }

    async fn set_execution_plan_job_failed(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        error: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_job_failed(&job_id.into(), &error)
            .await?;
        Ok(true)
    }
}
