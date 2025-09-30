use crate::context::BoscaContext;
use crate::datastores::security::WORKFLOW_MANAGERS_GROUP;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::graphql::workflows::activities::ActivitiesObject;
use crate::graphql::workflows::models::ModelsObject;
use crate::graphql::workflows::prompts::PromptsObject;
use crate::graphql::workflows::states::WorkflowStatesObject;
use crate::graphql::workflows::storage_systems::StorageSystemsObject;
use crate::graphql::workflows::traits::TraitsObject;
use crate::graphql::workflows::transitions::TransitionsObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_activity::WorkflowActivityObject;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::graphql::workflows::workflow_job::WorkflowJobObject;
use crate::graphql::workflows::workflow_schedules::WorkflowSchedulesObject;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::security::util::check_has_group;
use async_graphql::{Context, Error, Object, Union};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct WorkflowsObject {}

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
enum WorkflowExecution {
    Plan(WorkflowExecutionPlanObject),
    Job(WorkflowJobObject),
}

#[Object(name = "Workflows")]
impl WorkflowsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let states = ctx.workflow.get_workflows().await?;
        Ok(states.into_iter().map(WorkflowObject::new).collect())
    }

    async fn workflow(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<WorkflowObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let workflow = ctx.workflow.get_workflow(&id).await?;
        Ok(workflow.map(WorkflowObject::new))
    }

    async fn workflow_activity(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> Result<Option<WorkflowActivityObject>, Error> {
        check_has_group(ctx, WORKFLOW_MANAGERS_GROUP).await?;
        let ctx = ctx.data::<BoscaContext>()?;
        let workflow = ctx.workflow.get_workflow_activity(&id).await?;
        Ok(workflow.map(|a| WorkflowActivityObject::new(None, &a)))
    }

    async fn schedules(&self) -> WorkflowSchedulesObject {
        WorkflowSchedulesObject {}
    }

    async fn activities(&self) -> ActivitiesObject {
        ActivitiesObject {}
    }

    async fn models(&self) -> ModelsObject {
        ModelsObject {}
    }

    async fn prompts(&self) -> PromptsObject {
        PromptsObject {}
    }

    async fn states(&self) -> WorkflowStatesObject {
        WorkflowStatesObject {}
    }

    async fn transitions(&self) -> TransitionsObject {
        TransitionsObject {}
    }

    async fn traits(&self) -> TraitsObject {
        TraitsObject {}
    }

    async fn storage_systems(&self) -> StorageSystemsObject {
        StorageSystemsObject {}
    }

    async fn next_job(
        &self,
        ctx: &Context<'_>,
        queue: String,
    ) -> Result<Option<WorkflowJobObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(ctx
            .workflow
            .dequeue_next_execution(&queue)
            .await?
            .map(WorkflowJobObject::new))
    }

    #[allow(clippy::too_many_arguments)]
    async fn test_plan(
        &self,
        ctx: &Context<'_>,
        trait_id: Option<String>,
        profile_id: Option<String>,
        workflow_id: Option<String>,
        metadata_id: Option<String>,
        metadata_version: Option<i32>,
        comment_id: Option<i64>,
        collection_id: Option<String>,
        storage_system_ids: Option<Vec<String>>,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
        delay_until: Option<DateTime<Utc>>,
    ) -> Result<Option<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let mut request = EnqueueRequest {
            trait_id,
            profile_id: profile_id.map(|p| Uuid::parse_str(&p).unwrap()),
            workflow_id,
            workflow: None,
            metadata_id: metadata_id.map(|m| Uuid::parse_str(&m).unwrap()),
            metadata_version,
            comment_id,
            collection_id: collection_id.map(|c| Uuid::parse_str(&c).unwrap()),
            storage_system_ids: storage_system_ids.map(|s| {
                s.into_iter()
                    .map(|s| Uuid::parse_str(&s).unwrap())
                    .collect()
            }),
            configurations,
            delay_until,
            wait_for_completion: false,
        };
        Ok(Some(WorkflowExecutionPlanObject::new(
            ctx.workflow.get_new_execution_plan(&mut request).await?,
        )))
    }

    async fn execution_plan(
        &self,
        ctx: &Context<'_>,
        queue: String,
        id: String,
    ) -> Result<Option<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let id = Uuid::parse_str(&id)?;
        let id = WorkflowExecutionId { queue, id };
        Ok(ctx
            .workflow
            .get_execution_plan(&id)
            .await?
            .map(|p| p.into()))
    }

    async fn queues(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let queues = ctx.workflow.get_queues().await?;
        Ok(queues)
    }

    async fn executions(
        &self,
        ctx: &Context<'_>,
        queue: Option<String>,
        offset: i64,
        limit: i64,
        active: Option<bool>,
        failures: Option<bool>
    ) -> Result<Vec<WorkflowExecution>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let items = ctx
            .workflow
            .get_execution_plans(queue, offset, limit, active, failures)
            .await?;
        Ok(items
            .into_iter()
            .map(|plan| WorkflowExecution::Plan(WorkflowExecutionPlanObject::new(plan)))
            .collect())
    }
}
