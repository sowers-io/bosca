use std::collections::HashSet;
use std::str::FromStr;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::download::download_supplementary_path;
use bosca_client::upload::upload_multipart_supplementary_bytes;
use crate::ml::transcription::Transcription;

pub struct TranscriptionMapperActivity {
    id: String,
}

impl Default for TranscriptionMapperActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionMapperActivity {
    pub fn new() -> TranscriptionMapperActivity {
        TranscriptionMapperActivity {
            id: "transcription.mapper".to_string(),
        }
    }
}

#[async_trait]
impl Activity for TranscriptionMapperActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = job.metadata.as_ref().unwrap().id.to_owned();
        let inputs: HashSet<String> = job.workflow_activity.inputs.iter().map(|input| {
            input.value.to_owned()
        }).collect();

        let mut transcription: Option<Transcription> = None;
        let mut segments: Option<Vec<String>> = None;
        for supplementary in job.metadata.as_ref().unwrap().supplementary.iter() {
            if !inputs.contains(&supplementary.key) {
                continue;
            }
            let download = client.get_metadata_supplementary_download(&metadata_id, &supplementary.key).await?;
            if download.is_none() {
                return Err(Error::new("missing supplementary file".to_owned()));
            }
            let file = download_supplementary_path(&metadata_id, &download.unwrap()).await?;
            context.add_file_clean(&file);
            let source = supplementary.attributes.as_ref().unwrap().get("source").unwrap().as_str().unwrap();
            match source {
                "transcription" => {
                    let mut f = File::open(&file).await?;
                    let mut s = String::new();
                    f.read_to_string(&mut s).await?;
                    transcription = Some(Transcription::deserialize(Value::from_str(&s)?)?);
                }
                "segments" => {
                    let mut f = File::open(&file).await?;
                    let mut s = String::new();
                    f.read_to_string(&mut s).await?;
                    segments = Some(s.split("\n").map(|s| s.to_string()).collect());
                }
                _ => {}
            }
        }
        if let Some(transcription) = transcription {
            let mut new_transcription = Transcription {
                language: transcription.language.to_owned(),
                segments: vec![],
                text: "".to_string(),
            };
            if let Some(segments) = segments {
                for segment in segments.iter() {
                    let new_segment = transcription.get_segment(segment);
                    new_transcription.segments.push(new_segment);
                }
                let key = if job.workflow_activity.outputs.is_empty() {
                    "transcription_mapped".to_owned()
                } else {
                    job.workflow_activity.outputs.first().unwrap().value.to_owned()
                };
                if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == key) {
                    let mut attributes = Map::new();
                    attributes.insert("source".to_owned(), Value::String("transcription_mapper".to_owned()));
                    client.add_metadata_supplementary(MetadataSupplementaryInput {
                        metadata_id: metadata_id.to_owned(),
                        key: key.to_owned(),
                        attributes: Some(Value::Object(attributes)),
                        name: "Transcription Mapped".to_owned(),
                        content_type: "application/json".to_owned(),
                        content_length: None,
                        source_id: None,
                        source_identifier: None,
                    }).await?;
                }
                let upload_url = client.get_metadata_supplementary_upload(&metadata_id, &key).await?;
                let response_bytes = Bytes::from(json!(transcription).to_string());
                upload_multipart_supplementary_bytes(&metadata_id, "application/json", &upload_url, response_bytes).await?;
                Ok(())
            } else {
                Err(Error::new("missing segments".to_owned()))
            }
        } else {
            Err(Error::new("missing transcription".to_owned()))
        }
    }
}
