use crate::activity::{Activity, ActivityContext, Error};
use crate::media::transcriptions::transcription::{Transcription, TranscriptionResult};
use crate::metadata::foreach::{ForEachWorkflow, ForEachWorkflowActivity, ForEachWorkflows};
use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_supplementary_path;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::str::FromStr;
use bytes::Bytes;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::upload::upload_supplementary;

pub struct TranscriptionMapperToForEachActivity {
    id: String,
}

impl Default for TranscriptionMapperToForEachActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionMapperToForEachActivity {
    pub fn new() -> TranscriptionMapperToForEachActivity {
        TranscriptionMapperToForEachActivity {
            id: "transcription.mapper.foreach".to_string(),
        }
    }
}

#[async_trait]
impl Activity for TranscriptionMapperToForEachActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        context: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let metadata_id = job.metadata.as_ref().unwrap().id.to_owned();
        let inputs: HashSet<String> = job
            .workflow_activity
            .inputs
            .iter()
            .map(|input| input.value.to_owned())
            .collect();
        let mut transcription: Option<Transcription> = None;
        for supplementary in job.metadata.as_ref().unwrap().supplementary.iter() {
            if !inputs.contains(&supplementary.key) || supplementary.attributes.is_none() {
                continue;
            }
            let download = client
                .get_metadata_supplementary_download(&metadata_id, &supplementary.key)
                .await?;
            if download.is_none() {
                return Err(Error::new("missing supplementary file".to_owned()));
            }
            let file = download_supplementary_path(&metadata_id, &download.unwrap()).await?;
            context.add_file_clean(&file);
            let mut f = File::open(&file).await?;
            let mut s = String::new();
            f.read_to_string(&mut s).await?;
            let result = TranscriptionResult::deserialize(Value::from_str(&s)?)?;
            transcription = Some(result.transcription);
        }
        if let Some(transcription) = transcription {
            let mut workflows = ForEachWorkflows {
                workflows: Vec::new(),
            };
            for segment in transcription.segments.iter() {
                workflows.workflows.push(ForEachWorkflow {
                    id: "media.video.segments".to_string(),
                    activities: vec![ForEachWorkflowActivity {
                        id: "media.video.segment".to_string(),
                        configuration: json!({
                            "segment": json!(segment),
                            "start": segment.start,
                            "end": segment.end,
                        }),
                    }],
                });
            }
            let workflows_str = json!(workflows).to_string();
            upload_supplementary(client, job, "Mapper For Each", Bytes::from(workflows_str), Some("application/json".to_owned())).await?;
            Ok(())
        } else {
            Err(Error::new("missing transcription".to_owned()))
        }
    }
}
