use crate::context::{BoscaContext, PermissionCheck};
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::graphql::workflows::activities_mutation::ActivitiesMutationObject;
use crate::graphql::workflows::models_mutation::ModelsMutationObject;
use crate::graphql::workflows::prompts_mutation::PromptsMutationObject;
use crate::graphql::workflows::states_mutation::WorkflowStatesMutationObject;
use crate::graphql::workflows::storage_systems_mutation::StorageSystemsMutationObject;
use crate::graphql::workflows::traits_mutation::TraitsMutationObject;
use crate::graphql::workflows::transitions_mutation::TransitionsMutationObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::graphql::workflows::workflow_schedules_mutation::WorkflowSchedulesMutationObject;
use crate::models::content::find_query::FindQueryInput;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::{
    WorkflowExecutionIdInput, WorkflowJobId, WorkflowJobIdInput,
};
use crate::models::workflow::states::PENDING;
use crate::models::workflow::transitions::BeginTransitionInput;
use crate::models::workflow::workflows::WorkflowInput;
use crate::security::util::check_has_group;
use crate::util::transition::begin_transition;
use crate::workflow::core_workflow_ids::{
    COLLECTION_DELAYED_TRANSITION, METADATA_DELAYED_TRANSITION,
};
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use log::warn;
use serde_json::Value;
use uuid::Uuid;

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
        if let Some(workflow) = ctx
            .workflow
            .get_workflow(&workflow.id)
            .await?
            .map(WorkflowObject::new)
        {
            return Ok(workflow);
        }
        Err(Error::new(format!("missing workflow: {}", workflow.id)))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        workflow: WorkflowInput,
    ) -> Result<WorkflowObject, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.workflow.edit_workflow(&workflow).await?;
        if let Some(workflow) = ctx
            .workflow
            .get_workflow(&workflow.id)
            .await?
            .map(WorkflowObject::new)
        {
            return Ok(workflow);
        }
        Err(Error::new(format!("missing workflow: {}", workflow.id)))
    }

    async fn delete(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
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

    async fn storage_systems(&self) -> StorageSystemsMutationObject {
        StorageSystemsMutationObject {}
    }

    async fn schedules(&self) -> WorkflowSchedulesMutationObject {
        WorkflowSchedulesMutationObject {}
    }

    async fn expire_all(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow.expire_all().await?;
        Ok(true)
    }

    async fn retry_all_failed(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow.retry_all_failed().await?;
        Ok(true)
    }

    async fn retry_jobs(
        &self,
        ctx: &Context<'_>,
        id: Vec<WorkflowJobIdInput>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let ids = id
            .iter()
            .map(|id| WorkflowJobId {
                queue: id.queue.clone(),
                id: Uuid::parse_str(&id.id).expect("invalid id"),
                index: id.index,
            })
            .collect();
        ctx.workflow.retry_jobs(ids).await?;
        Ok(true)
    }

    async fn begin_transition(
        &self,
        ctx: &Context<'_>,
        request: BeginTransitionInput,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        begin_transition(ctx, &request, configurations).await?;
        Ok(true)
    }

    async fn cancel_transition(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        metadata_version: Option<i32>,
        collection_id: Option<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = &metadata_id {
            let id = Uuid::parse_str(metadata_id.as_str())?;
            if let Some(version) = metadata_version {
                let check = PermissionCheck::new_with_metadata_id_with_version(
                    id,
                    version,
                    PermissionAction::Edit,
                );
                let metadata = ctx.metadata_permission_check(check).await?;
                if metadata.workflow_state_id == PENDING {
                    ctx.content
                        .metadata_workflows
                        .set_metadata_not_ready(ctx, &metadata.id)
                        .await?;
                }
                ctx.workflow
                    .cancel_workflows(
                        &None,
                        &Some(METADATA_DELAYED_TRANSITION.to_string()),
                        &Some(id),
                        &metadata_version,
                        &None,
                    )
                    .await?;
                ctx.content
                    .metadata_workflows
                    .set_state(
                        ctx,
                        &ctx.principal,
                        &metadata,
                        &metadata.workflow_state_id,
                        None,
                        "Cancelled Transition",
                        false,
                        true,
                    )
                    .await?;
            }
        } else if let Some(collection_id) = &collection_id {
            let id = Uuid::parse_str(collection_id.as_str())?;
            let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Manage);
            let collection = ctx.collection_permission_check(check).await?;
            ctx.workflow
                .cancel_workflows(
                    &None,
                    &Some(COLLECTION_DELAYED_TRANSITION.to_string()),
                    &Some(id),
                    &metadata_version,
                    &None,
                )
                .await?;
            ctx.content
                .collection_workflows
                .set_state(
                    ctx,
                    &ctx.principal,
                    &collection,
                    &collection.workflow_state_id,
                    None,
                    "Cancelled Transition",
                    false,
                    true,
                )
                .await?;
        }
        Ok(true)
    }

    async fn cancel_workflows(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        workflow_id: Option<String>,
        metadata_id: Option<String>,
        metadata_version: Option<i32>,
        collection_id: Option<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let id = id.map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let metadata_id = metadata_id.map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let collection_id = collection_id.map(|id| Uuid::parse_str(id.as_str()).unwrap());
        ctx.workflow
            .cancel_workflows(
                &id,
                &workflow_id,
                &metadata_id,
                &metadata_version,
                &collection_id,
            )
            .await?;
        Ok(true)
    }

    async fn enqueue_child_workflows(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        workflow_ids: Vec<String>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<Vec<WorkflowExecutionIdObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(ctx
            .workflow
            .enqueue_job_child_workflows(&job_id.into(), &workflow_ids, delay_until)
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
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<WorkflowExecutionIdObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(WorkflowExecutionIdObject::new(
            ctx.workflow
                .enqueue_job_child_workflow(
                    &job_id.into(),
                    &workflow_id,
                    configurations,
                    delay_until,
                )
                .await?,
        ))
    }

    async fn find_and_enqueue_workflow(
        &self,
        ctx: &Context<'_>,
        workflow_id: String,
        query: FindQueryInput,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<Vec<WorkflowExecutionIdObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let mut ids = Vec::new();
        let mut request = EnqueueRequest {
            workflow_id: Some(workflow_id),
            configurations,
            delay_until,
            ..Default::default()
        };
        // TODO: page through items
        for metadata in ctx.content.metadata.find(&query).await? {
            request.metadata_id = Some(metadata.id);
            request.metadata_version = Some(metadata.version);
            let id = ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
            ids.push(id);
        }
        request.metadata_id = None;
        request.metadata_version = None;
        for collection in ctx.content.collections.find(&query).await? {
            request.collection_id = Some(collection.id);
            let id = ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
            ids.push(id);
        }
        Ok(ids
            .into_iter()
            .map(|plan| WorkflowExecutionIdObject::new(plan.first().unwrap().id.clone()))
            .collect())
    }

    #[allow(clippy::too_many_arguments)]
    async fn enqueue_workflow(
        &self,
        ctx: &Context<'_>,
        workflow_id: String,
        collection_id: Option<String>,
        metadata_id: Option<String>,
        profile_id: Option<String>,
        version: Option<i32>,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<WorkflowExecutionIdObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = metadata_id.map(|id| Uuid::parse_str(&id).unwrap());
        let collection_id = collection_id.map(|id| Uuid::parse_str(&id).unwrap());
        let profile_id = profile_id.map(|id| Uuid::parse_str(&id).unwrap());
        if let Some(metadata_id) = &metadata_id {
            let check =
                PermissionCheck::new_with_metadata_id(*metadata_id, PermissionAction::Edit);
            ctx.metadata_permission_check(check).await?;
        }
        if let Some(collection_id) = &collection_id {
            let check = PermissionCheck::new_with_collection_id(
                *collection_id,
                PermissionAction::Edit,
            );
            ctx.collection_permission_check(check).await?;
        }
        if profile_id.is_some() {
            ctx.check_has_service_account().await?;
        }
        let mut request = EnqueueRequest {
            workflow_id: Some(workflow_id),
            metadata_id,
            metadata_version: version,
            collection_id,
            profile_id,
            configurations,
            delay_until,
            ..Default::default()
        };
        let workflow = ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(WorkflowExecutionIdObject::new(
            workflow.first().unwrap().id.clone(),
        ))
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

    async fn set_execution_plan_job_context(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        context: Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_job_context(&job_id.into(), &context)
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

    async fn set_execution_plan_job_delayed(
        &self,
        ctx: &Context<'_>,
        job_id: WorkflowJobIdInput,
        delayed_until: DateTime<Utc>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        ctx.workflow
            .set_execution_plan_job_delayed(&job_id.into(), delayed_until)
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
        try_again: bool,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        warn!(
            "job failure reported: {} :: {} :: {} -> {}",
            job_id.queue, job_id.id, job_id.index, error
        );
        ctx.workflow
            .set_execution_plan_job_failed(&job_id.into(), &error, try_again)
            .await?;
        Ok(true)
    }
}
