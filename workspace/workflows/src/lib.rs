use crate::activity::{Activity, ActivityContext, Error};
use crate::metadata::index::IndexActivity;
use crate::metadata::traits::MetadataTraitsActivity;
use crate::metadata::transition_to::MetadataTransitionToActivity;
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::Arc;
use std::time::Duration;
use bosca_client::client::{Client, WorkflowExecution};
use crate::ai::prompt::PromptActivity;
use crate::collection::traits::CollectionTraitsActivity;
use crate::collection::transition_to::CollectionTransitionToActivity;
use crate::media::mux::MuxUploadActivity;
use crate::metadata::command::CommandActivity;
use media::transcriptions::mapper::TranscriptionMapperActivity;
use media::transcriptions::transcribe::TranscribeActivity;
use crate::metadata::foreach::MetadataForEachActivity;
use crate::metadata::tera::MetadataTeraActivity;

pub mod activity;
pub mod metadata;
pub mod collection;
pub mod media;
pub mod util;
pub mod ml;
pub mod ai;

pub fn get_default_activities() -> Vec<Box<dyn Activity + Send + Sync>> {
    vec![
        Box::new(CollectionTraitsActivity::default()),
        Box::new(CollectionTransitionToActivity::default()),
        Box::new(MetadataTraitsActivity::default()),
        Box::new(MetadataTransitionToActivity::default()),
        Box::new(IndexActivity::default()),
        Box::new(MuxUploadActivity::default()),
        Box::new(TranscribeActivity::default()),
        Box::new(TranscriptionMapperActivity::default()),
        Box::new(PromptActivity::default()),
        Box::new(CommandActivity::default()),
        Box::new(MetadataTeraActivity::default()),
        Box::new(MetadataForEachActivity::default()),
    ]
}

pub async fn process_queue(
    shutdown: Arc<AtomicBool>,
    activities_by_id: Arc<HashMap<String, Arc<Box<dyn Activity + Send + Sync>>>>,
    client: &Client,
    max_running: i32,
    queue: String,
) -> Result<(), Error> {
    info!(target: "workflow", "processing queue: {}, max: {}", queue, max_running);
    let running = Arc::new(AtomicI32::new(0));
    loop {
        if shutdown.load(Relaxed) {
            warn!("Queue {} is stopping because of shutdown signal", queue);
            break;
        }
        if running.load(Relaxed) >= max_running {
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
        if let Ok(Some(execution)) = client.get_next_execution(&queue).await {
            running.fetch_add(1, Relaxed);
            let activities_by_id = Arc::clone(&activities_by_id);
            let running = Arc::clone(&running);
            let client = client.clone();
            tokio::spawn(async move {
                match process(activities_by_id, &client, execution).await {
                    Ok(_) => {
                        running.fetch_add(-1, Relaxed);
                    }
                    Err(error) => {
                        error!(target: "workflow", "error: {}", error.to_string());
                        running.fetch_add(-1, Relaxed);
                    }
                }
            });
        } else {
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
    loop {
        if running.load(Relaxed) <= 0 {
            warn!("still waiting on activities to finish...");
            tokio::time::sleep(Duration::from_millis(5000)).await;
            return Ok(())
        }
    }
}

async fn process(
    activities_by_id: Arc<HashMap<String, Arc<Box<dyn Activity + Send + Sync>>>>,
    client: &Client,
    execution: WorkflowExecution,
) -> Result<(), Error> {
    match execution {
        bosca_client::client::plan::PlanWorkflowsNextWorkflowExecution::WorkflowExecutionPlan(plan) => {
            info!(target: "workflow", "processing execution plan: {} -> {}", plan.workflow.queue.clone(), plan.plan_id.clone());
            let id = plan.plan_id;
            let queue = plan.workflow.queue;
            let next_index = plan.next.unwrap().index;
            client.enqueue_job(id, &queue, next_index).await?;
        }
        bosca_client::client::plan::PlanWorkflowsNextWorkflowExecution::WorkflowJob(job) => {
            info!(target: "workflow", "processing execution job: {}", job.id.queue);
            let activity = activities_by_id.get(&job.activity.id);
            if activity.is_none() {
                error!(target: "workflow", "missing activity: {}", job.activity.id);
                let msg = format!("missing activity: {}", job.activity.id);
                if let Err(err) = client
                    .set_workflow_job_failed(job.id.id, job.id.index, &job.id.queue, &msg)
                    .await {
                    error!(target: "workflow", "failed to set job failed: {}, {}, {}, {} -- {} -- {}", job.id.id, job.id.index, job.id.queue, job.activity.id, msg, err);
                }
            } else {
                let activity = activity.unwrap();
                let mut context = ActivityContext::new();
                let keepalive_active = Arc::new(AtomicBool::new(true));
                let keepalive_id = job.id.clone();
                let keepalive_client = client.clone();
                let keepalive_active_loop = Arc::clone(&keepalive_active);
                let keepalive = tokio::spawn(async move {
                    loop {
                        if !keepalive_active_loop.load(Relaxed) {
                            break;
                        }
                        tokio::time::sleep(Duration::from_secs(3600)).await;
                        match keepalive_client
                            .set_workflow_job_checkin(
                                keepalive_id.id,
                                keepalive_id.index,
                                &keepalive_id.queue,
                            )
                            .await {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: "workflow", "failed to checkin: {}", e);
                                if e.to_string().contains("couldn't find message to update") {
                                    break;
                                }
                            }
                        }
                    }
                });
                match activity.execute(client, &mut context, &job).await {
                    Ok(_) => {
                        keepalive_active.store(false, Relaxed);
                        info!(target: "workflow", "job processed: {}, {}, {}, {}", job.id.id, job.id.index, job.id.queue, job.activity.id);
                        if let Err(err) = client
                            .set_workflow_job_complete(job.id.id, job.id.index, &job.id.queue)
                            .await {
                            error!(target: "workflow", "failed to set job complete: {}, {}, {}, {} -- {}", job.id.id, job.id.index, job.id.queue, job.activity.id, err);
                        }
                        let _ = context.close().await;
                    }
                    Err(err) => {
                        keepalive_active.store(false, Relaxed);
                        info!(target: "workflow", "job failed: {}", err);
                        let msg = err.to_string();
                        if let Err(err) = client
                            .set_workflow_job_failed(job.id.id, job.id.index, &job.id.queue, &msg)
                            .await {
                            error!(target: "workflow", "failed to set job failed: {}, {}, {}, {} -- {}", job.id.id, job.id.index, job.id.queue, job.activity.id, err);
                        }
                        let _ = context.close().await;
                    }
                }
                keepalive.abort();
            }
        }
    }
    Ok(())
}
