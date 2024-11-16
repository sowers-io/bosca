use std::collections::HashSet;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_supplementary_path;

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

        }

        todo!();
    }
}
