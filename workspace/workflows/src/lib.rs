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
use bosca_client::client::{Client, WorkflowJob};
use crate::ai::prompt::PromptActivity;
use crate::collection::traits::CollectionTraitsActivity;
use crate::collection::transition_to::CollectionTransitionToActivity;
use crate::media::mux::MuxUploadActivity;
use crate::metadata::command::CommandActivity;
use media::transcriptions::mapper::TranscriptionMapperActivity;
use media::transcriptions::transcribe::TranscribeActivity;
use crate::analytics::query::QueryActivity;
use crate::collection::begin_transition_to::CollectionBeginTransitionToActivity;
use crate::collection::delete::CollectionDeleteActivity;
use crate::collection::set_public::CollectionSetPublicActivity;
use crate::collection::set_ready::CollectionSetReadyActivity;
use crate::media::transcriptions::mapper_to_foreach::TranscriptionMapperToForEachActivity;
use crate::metadata::begin_transition_to::MetadataBeginTransitionToActivity;
use crate::metadata::delete::MetadataDeleteActivity;
use crate::metadata::foreach::MetadataForEachActivity;
use crate::metadata::set_public::MetadataSetPublicActivity;
use crate::metadata::set_ready::MetadataSetReadyActivity;
use crate::metadata::tera::MetadataTeraActivity;

pub mod activity;
pub mod metadata;
pub mod collection;
pub mod media;
pub mod util;
pub mod ml;
pub mod ai;
pub mod analytics;

pub fn get_default_activities() -> Vec<Box<dyn Activity + Send + Sync>> {
    vec![
        Box::new(CollectionTraitsActivity::default()),
        Box::new(CollectionTransitionToActivity::default()),
        Box::new(CollectionSetReadyActivity::default()),
        Box::new(CollectionSetPublicActivity::default()),
        Box::new(CollectionDeleteActivity::default()),
        Box::new(CollectionBeginTransitionToActivity::default()),
        Box::new(MetadataTraitsActivity::default()),
        Box::new(MetadataTransitionToActivity::default()),
        Box::new(MetadataTeraActivity::default()),
        Box::new(MetadataForEachActivity::default()),
        Box::new(MetadataSetReadyActivity::default()),
        Box::new(MetadataSetPublicActivity::default()),
        Box::new(MetadataDeleteActivity::default()),
        Box::new(MetadataBeginTransitionToActivity::default()),
        Box::new(IndexActivity::default()),
        Box::new(MuxUploadActivity::default()),
        Box::new(TranscribeActivity::default()),
        Box::new(TranscriptionMapperActivity::default()),
        Box::new(TranscriptionMapperToForEachActivity::default()),
        Box::new(PromptActivity::default()),
        Box::new(CommandActivity::default()),
        Box::new(QueryActivity::default()),

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
            continue;
        }
        match client.get_next_job(&queue).await {
            Ok(Some(job)) => {
                running.fetch_add(1, Relaxed);
                let activities_by_id = Arc::clone(&activities_by_id);
                let running = Arc::clone(&running);
                let client = client.clone();
                tokio::spawn(async move {
                    match process(activities_by_id, &client, job).await {
                        Ok(_) => {
                            running.fetch_add(-1, Relaxed);
                        }
                        Err(error) => {
                            error!(target: "workflow", "error: {}", error.to_string());
                            running.fetch_add(-1, Relaxed);
                        }
                    }
                });
            }
            Ok(None) => {
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
            Err(e) => {
                error!(target: "workflow", "error getting pending events: {}", e);
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
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
    job: WorkflowJob,
) -> Result<(), Error> {
    info!(target: "workflow", "processing job: {}", job.id.queue);
    let activity = activities_by_id.get(&job.activity.id);
    if activity.is_none() {
        error!(target: "workflow", "missing activity: {}", job.activity.id);
        let msg = format!("missing activity: {}", job.activity.id);
        if let Err(err) = client
            .set_workflow_job_failed(&job.id.id, job.id.index, &job.id.queue, &msg)
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
                tokio::time::sleep(Duration::from_secs(600)).await;
                match keepalive_client
                    .set_workflow_job_checkin(
                        &keepalive_id.id,
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
                    .set_workflow_job_complete(&job.id.id, job.id.index, &job.id.queue)
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
                    .set_workflow_job_failed(&job.id.id, job.id.index, &job.id.queue, &msg)
                    .await {
                    error!(target: "workflow", "failed to set job failed: {}, {}, {}, {} -- {}", job.id.id, job.id.index, job.id.queue, job.activity.id, err);
                }
                let _ = context.close().await;
            }
        }
        keepalive.abort();
    }
    Ok(())
}
