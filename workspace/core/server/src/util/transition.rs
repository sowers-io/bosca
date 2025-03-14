use crate::context::BoscaContext;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::item::ContentItem;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use crate::models::workflow::states::WorkflowStateType;
use crate::models::workflow::transitions::BeginTransitionInput;
use crate::workflow::core_workflow_ids::{COLLECTION_DELAYED_TRANSITION, METADATA_DELAYED_TRANSITION};
use async_graphql::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub async fn verify_transition_exists(
    ctx: &BoscaContext,
    state_id: &str,
    next_state_id: &str,
) -> Result<(), Error> {
    if ctx
        .workflow
        .get_transition(state_id, next_state_id)
        .await?
        .is_some()
    {
        Ok(())
    } else {
        Err(Error::new(format!(
            "transition doesn't exist: {} -> {}",
            state_id, next_state_id
        )))
    }
}

#[derive(Eq, PartialEq)]
pub enum TransitionType {
    Default,
    Enter,
    Exit,
}

pub async fn begin_transition(
    ctx: &BoscaContext,
    request: &BeginTransitionInput,
    configurations: Option<Vec<WorkflowConfigurationInput>>,
) -> Result<(), Error> {
    if let Some(metadata_id) = &request.metadata_id {
        let id = Uuid::parse_str(metadata_id.as_str())?;
        if let Some(version) = request.version {
            let metadata = ctx
                .check_metadata_version_action(&id, version, PermissionAction::Manage)
                .await?;
            if (request.restart.is_none() || !request.restart.unwrap())
                && metadata.workflow_state_id == request.state_id
            {
                return Err(Error::new("metadata is already in this state"));
            }
            verify_transition_exists(ctx, &metadata.workflow_state_id, &request.state_id).await?;
            // ensure we can exit our state
            transition(
                ctx,
                TransitionType::Exit,
                &metadata,
                &request.state_id,
                &None,
                Some(true),
                None,
                request.restart,
            )
            .await?;
            // ensure we can enter our state
            transition(
                ctx,
                TransitionType::Enter,
                &metadata,
                &request.state_id,
                &None,
                Some(true),
                None,
                request.restart,
            )
            .await?;
            // log what's about to happen
            ctx.content
                .metadata_workflows
                .set_metadata_workflow_state(
                    &ctx.principal,
                    &metadata,
                    &request.state_id,
                    request.state_valid,
                    &request.status,
                    true,
                    false,
                )
                .await?;
            // check to see if what is about to happen should be delayed
            let mut delay = false;
            if let Some(state_valid) = request.state_valid {
                if state_valid > Utc::now() {
                    delay = true;
                }
            }
            // do what should happen
            match transition(
                ctx,
                TransitionType::Default,
                &metadata,
                &request.state_id,
                &None,
                request.wait_for_completion,
                request.state_valid,
                request.restart,
            )
            .await?
            {
                None => {
                    // no plan was created because possibly need to be delayed (other times it can be because there's actually no plan to be created)
                    if delay {
                        let mut request = EnqueueRequest {
                            workflow_id: Some(METADATA_DELAYED_TRANSITION.to_string()),
                            metadata_id: Some(metadata.id),
                            metadata_version: Some(metadata.version),
                            delay_until: request.state_valid,
                            ..Default::default()
                        };
                        // enqueue a workflow that will re-run our transition later
                        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
                    } else {
                        // we don't need to delay, so just mark the transition as complete
                        ctx.content
                            .metadata_workflows
                            .set_metadata_workflow_state(
                                &ctx.principal,
                                &metadata,
                                &request.state_id,
                                request.state_valid,
                                &request.status,
                                true,
                                true,
                            )
                            .await?;
                    }
                }
                Some(_) => {
                    // something got planned!
                }
            }
        } else {
            return Err(Error::new("a metadata version is required"));
        }
    } else if let Some(collection_id) = &request.collection_id {
        let id = Uuid::parse_str(collection_id.as_str())?;
        let collection = ctx
            .check_collection_action(&id, PermissionAction::Manage)
            .await?;
        if (request.restart.is_none() || !request.restart.unwrap())
            && collection.workflow_state_id == request.state_id
        {
            return Err(Error::new("collection is already in this state"));
        }
        verify_transition_exists(ctx, &collection.workflow_state_id, &request.state_id).await?;
        // ensure we can exit our state
        transition(
            ctx,
            TransitionType::Exit,
            &collection,
            &request.state_id,
            &configurations,
            Some(true),
            None,
            request.restart,
        )
        .await?;
        // ensure we can enter our state
        transition(
            ctx,
            TransitionType::Enter,
            &collection,
            &request.state_id,
            &configurations,
            Some(true),
            None,
            request.restart,
        )
        .await?;
        // log what's about to happen
        ctx.content
            .collection_workflows
            .set_state(
                &ctx.principal,
                &collection,
                &request.state_id,
                request.state_valid,
                &request.status,
                true,
                false,
            )
            .await?;
        // check to see if what is about to happen should be delayed
        let mut delay = false;
        if let Some(state_valid) = request.state_valid {
            if state_valid > Utc::now() {
                delay = true;
            }
        }
        // do what should happen
        match transition(
            ctx,
            TransitionType::Default,
            &collection,
            &request.state_id,
            &configurations,
            request.wait_for_completion,
            request.state_valid,
            request.restart,
        )
        .await?
        {
            None => {
                // no plan was created because possibly need to be delayed (other times it can be because there's actually no plan to be created)
                if delay {
                    // enqueue a workflow that will re-run our transition later
                    let mut request = EnqueueRequest {
                        workflow_id: Some(COLLECTION_DELAYED_TRANSITION.to_string()),
                        collection_id: Some(collection.id),
                        delay_until: request.state_valid,
                        ..Default::default()
                    };
                    ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
                } else {
                    // we don't need to delay, so just mark the transition as complete
                    ctx.content
                        .collection_workflows
                        .set_state(
                            &ctx.principal,
                            &collection,
                            &request.state_id,
                            request.state_valid,
                            &request.status,
                            true,
                            true,
                        )
                        .await?;
                }
            }
            Some(_) => {
                // something got planned!
            }
        }
    } else {
        return Err(Error::new(
            "you must provide either a collection_id or a metadata_id",
        ));
    };
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn transition(
    ctx: &BoscaContext,
    transition_type: TransitionType,
    item: &impl ContentItem,
    next_state_id: &str,
    configurations: &Option<Vec<WorkflowConfigurationInput>>,
    wait_for_completion: Option<bool>,
    delay_until: Option<DateTime<Utc>>,
    restart: Option<bool>,
) -> Result<Option<WorkflowExecutionPlan>, Error> {
    if let Some(state) = ctx.workflow.get_state(item.workflow_state_id()).await? {
        if state.state_type == WorkflowStateType::Pending {
            return Err(Error::new("manual transition to processing isn't allowed, please mark as ready instead and wait for the item to be transitioned to draft"));
        }
    }
    if item.ready().is_none() {
        return Err(Error::new(
            "please mark as ready before transitioning to a new state",
        ));
    }
    if (restart.is_none() || !restart.unwrap()) && item.workflow_state_pending_id().is_some() {
        return Err(Error::new(
            "cannot transition due to existing pending state id",
        ));
    }
    if let Some(state) = ctx
        .workflow
        .get_state(if transition_type == TransitionType::Exit {
            item.workflow_state_id()
        } else {
            next_state_id
        })
        .await?
    {
        Ok(
            if let Some(workflow_id) = match transition_type {
                TransitionType::Enter => state.entry_workflow_id,
                TransitionType::Exit => state.exit_workflow_id,
                TransitionType::Default => state.workflow_id,
            } {
                let Some(workflow) = ctx.workflow.get_workflow(&workflow_id).await? else {
                    return Err(Error::new("missing workflow"));
                };
                let mut request = EnqueueRequest {
                    workflow: Some(workflow),
                    configurations: configurations.clone(),
                    wait_for_completion,
                    delay_until,
                    ..Default::default()
                };
                let workflows = if let Some(version) = item.version() {
                    request.metadata_id = Some(*item.id());
                    request.metadata_version = Some(version);
                    ctx.workflow.enqueue_workflow(ctx, &mut request).await?
                } else {
                    request.collection_id = Some(*item.id());
                    ctx.workflow.enqueue_workflow(ctx, &mut request).await?
                };
                workflows.into_iter().next()
            } else {
                None
            },
        )
    } else {
        Err(Error::new("state doesn't exist"))
    }
}
