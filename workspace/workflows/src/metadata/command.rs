use std::collections::{HashMap, HashSet};
use std::str::from_utf8;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use serde_json::{json, Value};
use tokio::process::Command;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::download::{download_path, download_supplementary_path};
use bosca_client::upload::upload_multipart_supplementary_bytes;

pub struct CommandActivity {
    id: String,
}

impl Default for CommandActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandActivity {
    pub fn new() -> CommandActivity {
        CommandActivity {
            id: "metadata.command".to_string(),
        }
    }
}

#[async_trait]
impl Activity for CommandActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = job.metadata.as_ref().unwrap().id.to_owned();
        let command = job.activity.configuration.get("command").unwrap().as_str().unwrap().to_owned();
        let command_args = job.activity.configuration.get("command_args").unwrap().as_array().unwrap().iter().map(|arg| arg.as_str().unwrap().to_owned()).collect::<Vec<String>>();
        let include_metadata = job.activity.configuration.get("include_metadata").unwrap_or(&Value::Bool(true)).as_bool().unwrap();

        let inputs: HashSet<String> = job.workflow_activity.inputs.iter().map(|input| {
            input.value.to_owned()
        }).collect();

        let job_json = json!(job).to_string();
        let job_file = context.write_to_file(job_json.as_bytes()).await?;

        let mut files = HashMap::new();
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
            files.insert(supplementary.key.to_owned(), file);
        }

        let mut cmd = Command::new(command);
        cmd.args(command_args);
        cmd.env("BOSCA_JOB", job_file);

        if include_metadata {
            let download = client.get_metadata_download_url(&metadata_id).await?;
            let metadata_file = download_path(&metadata_id, &download).await?;
            context.add_file_clean(&metadata_file);
            cmd.env("BOSCA_METADATA", &metadata_file);
        }

        for (key, file) in files.iter() {
            cmd.env(format!("BOSCA_SUPPLEMENTARY_{}", key), file);
        }
        let output = cmd.output().await?;

        if !output.stderr.is_empty() {
            let err = from_utf8(&output.stderr).map_err(|e| Error::new(format!("error converting stderr: {}", e)))?;
            return Err(Error::new(format!("stderr: {}", err)));
        }
        if !output.status.success() {
            return Err(Error::new(format!("invalid exit code: {:?}", output.status.code())));
        }

        if !job.activity.outputs.is_empty() {
            let content_type = job.activity.configuration.get("command_content_type");
            let mime_type = if content_type.is_some() {
                content_type.unwrap().as_str().unwrap().to_owned()
            } else {
                let kind = infer::get(&output.stdout);
                if kind.is_some() {
                    kind.unwrap().mime_type().to_owned()
                } else {
                    "application/octet-stream".to_owned()
                }
            };
            let key = &job.activity.outputs.first().unwrap().name;
            if job.metadata.as_ref().unwrap().supplementary.is_empty() {
                client.add_metadata_supplementary(MetadataSupplementaryInput {
                    metadata_id: metadata_id.to_owned(),
                    key: key.to_owned(),
                    name: "Command Output".to_owned(),
                    content_type: mime_type.to_owned(),
                    content_length: None,
                    source_id: None,
                    source_identifier: None,
                }).await?;
            }
            let upload_url = client.get_metadata_supplementary_upload(&metadata_id, key).await?;
            upload_multipart_supplementary_bytes(&metadata_id, &mime_type, &upload_url, Bytes::from(output.stdout)).await?;
        }

        Ok(())
    }
}
