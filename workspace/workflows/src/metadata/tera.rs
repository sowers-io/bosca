use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_supplementary_path;
use bosca_client::upload::upload_supplementary;
use bytes::Bytes;
use serde_json::Value;
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct MetadataTeraActivity {
    id: String,
}

impl Default for MetadataTeraActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataTeraActivity {
    pub fn new() -> MetadataTeraActivity {
        MetadataTeraActivity {
            id: "metadata.tera".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataTeraActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        context: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let metadata = job.metadata.as_ref().unwrap();
        let metadata_id = &metadata.id;
        let input = &job.workflow_activity.inputs.first().unwrap().value;
        let download = client
            .get_metadata_supplementary_download(metadata_id, input)
            .await?
            .unwrap();
        let file = download_supplementary_path(metadata_id, &download).await?;
        context.add_file_clean(&file);

        let mut f = File::open(&file).await?;
        let mut s = String::new();
        f.read_to_string(&mut s).await?;
        let value: Value = serde_json::from_str(&s)?;

        let template = job
            .workflow_activity
            .configuration
            .get("template")
            .unwrap()
            .as_str()
            .unwrap();
        let content_type = job
            .workflow_activity
            .configuration
            .get("content_type")
            .unwrap()
            .as_str()
            .unwrap();
        let mut tera = Tera::default();
        tera.add_raw_template("template", template)
            .map_err(|e| Error::new(format!("Template generation error: {}", e)))?;

        let tera_context = Context::from_value(value)
            .map_err(|e| Error::new(format!("Template context error: {}", e)))?;
        let result = tera
            .render("template", &tera_context)
            .map_err(|e| Error::new(format!("Template render error: {}", e)))?;

        upload_supplementary(client, job, "Template Result", Bytes::from(result), Some(content_type.parse().unwrap())).await?;

        Ok(())
    }
}
