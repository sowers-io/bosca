use std::collections::{HashMap, HashSet};
use std::str::from_utf8;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use log::info;
use serde_json::{json, Map, Value};
use tokio::fs::File;
use tokio::process::Command;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::download::{download_path, download_path_with_extension, download_supplementary_path};
use bosca_client::upload::{upload_multipart_supplementary_bytes, upload_multipart_supplementary_file};

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
        let command = job.workflow_activity.configuration.get("command").unwrap().as_str().unwrap().to_owned();
        let input_file_ext = job.workflow_activity.configuration.get("input_ext").unwrap_or(&Value::String("".to_owned())).as_str().unwrap().to_owned();
        let output_file_ext = job.workflow_activity.configuration.get("output_ext").unwrap_or(&Value::String("".to_owned())).as_str().unwrap().to_owned();

        let include_metadata = job.workflow_activity.configuration.get("include_metadata").unwrap_or(&Value::Bool(true)).as_bool().unwrap();

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

        let metadata_file = if include_metadata {
            let download = client.get_metadata_download_url(&metadata_id).await?;
            let metadata_file = if input_file_ext.is_empty() {
                download_path(&metadata_id, &download).await?
            } else {
                download_path_with_extension(&metadata_id, &download, Some(input_file_ext)).await?
            };
            context.add_file_clean(&metadata_file);
            Some(metadata_file)
        } else {
            None
        };

        let output_file = context.new_file(&output_file_ext).await?;
        let command_args = job.workflow_activity.configuration.get("command_args").unwrap().as_array().unwrap().iter().map(|arg| {
            let arg = arg.as_str().unwrap().to_owned();
            if arg.starts_with("$") {
                let name = &arg.as_str()[1..];
                match name {
                    "BOSCA_JOB" => job_file.to_owned(),
                    "BOSCA_METADATA" => if let Some(metadata_file) = metadata_file.as_ref() {
                        metadata_file.to_owned()
                    } else {
                        "".to_owned()
                    },
                    "BOSCA_OUTPUT_FILE" => output_file.to_owned(),
                    _ => {
                        if let Some(key) = name.strip_prefix("BOSCA_SUPPLEMENTARY_") {
                            if files.contains_key(key) {
                                files.get(key).unwrap().to_owned()
                            } else {
                                "".to_owned()
                            }
                        } else {
                            arg
                        }
                    }
                }
            } else {
                arg
            }
        }).collect::<Vec<String>>();

        let mut cmd = Command::new(command);
        cmd.args(command_args);
        cmd.env("BOSCA_JOB", job_file);
        if let Some(metadata_file) = metadata_file {
            cmd.env("BOSCA_METADATA", &metadata_file);
        }
        cmd.env("BOSCA_OUTPUT_FILE", &output_file);

        for (key, file) in files.iter() {
            cmd.env(format!("BOSCA_SUPPLEMENTARY_{}", key), file);
        }

        info!("{:?}", cmd);

        let output = cmd.output().await?;

        if !output.stderr.is_empty() {
            let err = from_utf8(&output.stderr).map_err(|e| Error::new(format!("error converting stderr: {}", e)))?;
            return Err(Error::new(format!("stderr: {}", err)));
        }
        if !output.status.success() {
            return Err(Error::new(format!("invalid exit code: {:?}", output.status.code())));
        }
        if !job.activity.outputs.is_empty() {
            let key = job.workflow_activity.outputs.first().unwrap().value.to_owned();
            let content_type = job.activity.configuration.get("command_content_type");
            if tokio::fs::try_exists(&output_file).await? {
                let mime_type = if content_type.is_some() {
                    content_type.unwrap().as_str().unwrap().to_owned()
                } else {
                    let kind = infer::get_from_path(&output_file)?;
                    if kind.is_some() {
                        kind.unwrap().mime_type().to_owned()
                    } else {
                        "application/octet-stream".to_owned()
                    }
                };
                if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == key) {
                    let mut attributes = Map::new();
                    if let Some(source) = job.workflow_activity.configuration.get("source") {
                        attributes.insert("source".to_owned(), source.clone());
                    }
                    client.add_metadata_supplementary(MetadataSupplementaryInput {
                        metadata_id: metadata_id.to_owned(),
                        key: key.to_owned(),
                        attributes: Some(Value::Object(attributes)),
                        name: "Command Output".to_owned(),
                        content_type: mime_type.to_owned(),
                        content_length: None,
                        source_id: None,
                        source_identifier: None,
                    }).await?;
                }
                let file = File::open(&output_file).await?;
                let upload_url = client.get_metadata_supplementary_upload(&metadata_id, &key).await?;
                upload_multipart_supplementary_file(&metadata_id, &mime_type, &upload_url, file).await?;
            } else {
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
                if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == key) {
                    let mut attributes = Map::new();
                    if let Some(source) = job.workflow_activity.configuration.get("source") {
                        attributes.insert("source".to_owned(), source.clone());
                    }
                    client.add_metadata_supplementary(MetadataSupplementaryInput {
                        metadata_id: metadata_id.to_owned(),
                        key: key.to_owned(),
                        attributes: Some(Value::Object(attributes)),
                        name: "Command Output".to_owned(),
                        content_type: mime_type.to_owned(),
                        content_length: None,
                        source_id: None,
                        source_identifier: None,
                    }).await?;
                }
                let upload_url = client.get_metadata_supplementary_upload(&metadata_id, &key).await?;
                upload_multipart_supplementary_bytes(&metadata_id, &mime_type, &upload_url, Bytes::from(output.stdout)).await?;
            }
        }

        Ok(())
    }
}
