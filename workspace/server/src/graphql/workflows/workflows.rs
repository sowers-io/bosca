use crate::graphql::workflows::activities::ActivitiesObject;
use crate::graphql::workflows::models::ModelsObject;
use crate::graphql::workflows::prompts::PromptsObject;
use crate::graphql::workflows::states::WorkflowStatesObject;
use crate::graphql::workflows::storage_systems::StorageSystemsObject;
use crate::graphql::workflows::transition::TransitionObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::graphql::workflows::workflow_job::WorkflowJobObject;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::MessageValue;
use async_graphql::{Context, Error, Object, Union};
use crate::context::BoscaContext;

pub(crate) struct WorkflowsObject {}

const ACTIVITIES: ActivitiesObject = ActivitiesObject {};
const MODELS: ModelsObject = ModelsObject {};
const PROMPTS: PromptsObject = PromptsObject {};
const STATES: WorkflowStatesObject = WorkflowStatesObject {};
const STORAGE_SYSTEMS: StorageSystemsObject = StorageSystemsObject {};

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
enum WorkflowExecution {
    Plan(WorkflowExecutionPlanObject),
    Job(WorkflowJobObject),
}

#[Object(name = "Workflows")]
impl WorkflowsObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let states = ctx.workflow.get_workflows().await?;
        Ok(states.into_iter().map(WorkflowObject::new).collect())
    }

    async fn activities(&self) -> &ActivitiesObject {
        &ACTIVITIES
    }

    async fn models(&self) -> &ModelsObject {
        &MODELS
    }

    async fn prompts(&self) -> &PromptsObject {
        &PROMPTS
    }

    async fn states(&self) -> &WorkflowStatesObject {
        &STATES
    }

    async fn storage_systems(&self) -> &StorageSystemsObject {
        &STORAGE_SYSTEMS
    }

    async fn transitions(&self, ctx: &Context<'_>) -> Result<Vec<TransitionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let states = ctx.workflow.get_transitions().await?;
        Ok(states.into_iter().map(TransitionObject::new).collect())
    }

    async fn next_workflow_execution(
        &self,
        ctx: &Context<'_>,
        queue: String,
    ) -> Result<Option<WorkflowExecution>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let message = ctx.workflow.dequeue_next_execution(&queue).await?;
        if message.is_none() {
            return Ok(None)
        }
        Ok(message.map(|execution| match execution {
            MessageValue::Plan(plan) => {
                WorkflowExecution::Plan(WorkflowExecutionPlanObject::new(plan))
            }
            MessageValue::Job(job) => WorkflowExecution::Job(WorkflowJobObject::new(job)),
        }))
    }

    async fn execution_plan(
        &self,
        ctx: &Context<'_>,
        queue: String,
        id: i64,
    ) -> Result<Option<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let id = WorkflowExecutionId { queue, id };
        Ok(ctx.workflow.get_execution_plan(&id).await?.map(|p| p.into()))
    }

    async fn executions(
        &self,
        ctx: &Context<'_>,
        queue: String,
        offset: i64,
        limit: i64,
        archived: bool,
    ) -> Result<Vec<WorkflowExecution>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        Ok(ctx.workflow
            .get_execution_plans(&queue, offset, limit, archived)
            .await?
            .into_iter()
            .map(|execution| match execution {
                MessageValue::Plan(plan) => {
                    WorkflowExecution::Plan(WorkflowExecutionPlanObject::new(plan))
                }
                MessageValue::Job(job) => WorkflowExecution::Job(WorkflowJobObject::new(job)),
            })
            .collect())
    }
}
