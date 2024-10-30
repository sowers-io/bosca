use async_graphql::Error;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::item::ContentItem;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use crate::models::workflow::states::WorkflowStateType;
use crate::models::workflow::transitions::BeginTransitionInput;

pub async fn verify_transition_exists(ctx: &BoscaContext, state_id: &str, next_state_id: &str) -> Result<(), Error> {
    if ctx.workflow.get_transition(state_id, next_state_id).await?.is_some() {
        Ok(())
    } else {
        Err(Error::new("transition doesn't exist"))
    }
}

#[derive(Eq, PartialEq)]
pub enum TransitionType {
    Default,
    Enter,
    Exit,
}

pub async fn begin_transition(ctx: &BoscaContext, request: &BeginTransitionInput, configurations: Option<&Vec<WorkflowConfigurationInput>>) -> Result<(), Error> {
    if let Some(metadata_id) = &request.metadata_id {
        let id = Uuid::parse_str(metadata_id.as_str())?;
        if let Some(version) = request.version {
            let metadata = ctx.check_metadata_version_action(&id, version, PermissionAction::Manage).await?;
            if metadata.workflow_state_id == request.state_id {
                return Err(Error::new("metadata is already in this state"));
            }
            verify_transition_exists(ctx, &metadata.workflow_state_id, &request.state_id).await?;
            transition(ctx, TransitionType::Exit, &metadata, &request.state_id, None, Some(false)).await?;
            transition(ctx, TransitionType::Enter, &metadata, &request.state_id, None, Some(false)).await?;
            ctx.content.set_metadata_workflow_state(&ctx.principal, &metadata, &request.state_id, "User Request", true, false).await?;
            if transition(ctx, TransitionType::Default, &metadata, &request.state_id, None, request.wait_for_completion).await?.is_none() {
                ctx.content.set_metadata_workflow_state(&ctx.principal, &metadata, &request.state_id, "User Request", true, true).await?;
            }
        } else {
            return Err(Error::new("a version is required"));
        }
    } else if let Some(collection_id) = &request.collection_id {
        let id = Uuid::parse_str(collection_id.as_str())?;
        let collection = ctx.check_collection_action(&id, PermissionAction::Manage).await?;
        if collection.workflow_state_id == request.state_id {
            return Err(Error::new("collection is already in this state"));
        }
        verify_transition_exists(ctx, &collection.workflow_state_id, &request.state_id).await?;
        transition(ctx, TransitionType::Exit, &collection, &request.state_id, configurations, Some(false)).await?;
        transition(ctx, TransitionType::Enter, &collection, &request.state_id, configurations, Some(false)).await?;
        ctx.content.set_collection_workflow_state(&ctx.principal, &collection, &request.state_id, "User Request", true, false).await?;
        if transition(ctx, TransitionType::Default, &collection, &request.state_id, configurations, request.wait_for_completion).await?.is_none() {
            ctx.content.set_collection_workflow_state(&ctx.principal, &collection, &request.state_id, "User Request", true, true).await?;
        }
    } else {
        return Err(Error::new("you must provide either a collection_id or a metadata_id"));
    };
    Ok(())
}

pub async fn transition(ctx: &BoscaContext, transition_type: TransitionType, item: &impl ContentItem, next_state_id: &str, configurations: Option<&Vec<WorkflowConfigurationInput>>, wait_for_completion: Option<bool>) -> Result<Option<WorkflowExecutionPlan>, Error> {
    if let Some(state) = ctx.workflow.get_state(item.workflow_state_id()).await? {
        if state.state_type == WorkflowStateType::Pending {
            return Err(Error::new("manual transition to processing isn't allowed, please mark as ready instead and wait for the item to be transitioned to draft"));
        }
    }
    if item.ready().is_none() {
        return Err(Error::new("please mark as ready before transitioning to a new state"));
    }
    if item.workflow_state_pending_id().is_some() {
        return Err(Error::new("cannot transition due to existing pending state id"));
    }
    if let Some(state) = ctx.workflow.get_state(if transition_type == TransitionType::Exit {
        item.workflow_state_id()
    } else {
        next_state_id
    }).await? {
        Ok(if let Some(workflow_id) = match transition_type {
            TransitionType::Enter => state.entry_workflow_id,
            TransitionType::Exit => state.exit_workflow_id,
            TransitionType::Default => state.workflow_id,
        } {
            Some(if let Some(version) = item.version() {
                ctx.workflow.enqueue_metadata_workflow(
                    &workflow_id,
                    item.id(),
                    &version,
                    configurations,
                    wait_for_completion,
                ).await?
            } else {
                ctx.workflow.enqueue_collection_workflow(
                    &workflow_id,
                    item.id(),
                    configurations,
                    wait_for_completion,
                ).await?
            })
        } else {
            None
        })
    } else {
        Err(Error::new("state doesn't exist"))
    }
}
