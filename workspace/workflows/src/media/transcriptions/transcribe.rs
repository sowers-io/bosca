use std::env;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use serde_json::{Map, Value};
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::upload::upload_multipart_supplementary_bytes;
use crate::ml::runpod::execute_runpod;

pub struct TranscribeActivity {
    id: String,
}

impl Default for TranscribeActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscribeActivity {
    pub fn new() -> TranscribeActivity {
        TranscribeActivity {
            id: "media.transcribe".to_string(),
        }
    }
}

#[async_trait]
impl Activity for TranscribeActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, _context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let key = if job.workflow_activity.outputs.is_empty() {
            "transcription".to_owned()
        } else {
            job.workflow_activity.outputs.first().unwrap().value.to_owned()
        };
        if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == key) {
            let mut attributes = Map::new();
            attributes.insert("source".to_owned(), Value::String("transcription".to_owned()));
            client.add_metadata_supplementary(MetadataSupplementaryInput {
                metadata_id: metadata_id.to_owned(),
                key: key.to_owned(),
                attributes: Some(Value::Object(attributes)),
                name: "Transcription".to_owned(),
                content_type: "application/json".to_owned(),
                content_length: None,
                source_id: None,
                source_identifier: None,
            }).await?;
        }
        let upload_url = client.get_metadata_supplementary_upload(metadata_id, &key).await?;
        let transcribe_function = env::var("RUNPOD_TRANSCRIBE_FUNCTION")?;
        let response = execute_runpod(client, &transcribe_function, "transcribe", job).await?;
        let response_bytes = Bytes::from(response.to_string());
        upload_multipart_supplementary_bytes(metadata_id, "application/json", &upload_url, response_bytes).await?;
        Ok(())
    }
}
