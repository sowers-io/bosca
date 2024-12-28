use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use std::collections::{HashMap};
use std::str::from_utf8;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_search_documents::SearchDocumentInput;
use bosca_client::download::{download_path, download_supplementary_path};

pub struct IndexActivity {
    id: String,
}

impl Default for IndexActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexActivity {
    pub fn new() -> IndexActivity {
        IndexActivity {
            id: "metadata.text.index".to_string(),
        }
    }
}

#[async_trait]
impl Activity for IndexActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata = job.metadata.as_ref().unwrap();
        let inputs: HashMap<_, _> = job.workflow_activity.inputs.iter().map(|i| (&i.name, i)).collect();
        let supplementary_key = "supplementaryId".to_string();
        let path: String;
        if inputs.contains_key(&supplementary_key) {
            let supplementary_id = inputs.get(&supplementary_key).unwrap().value.to_string();
            let download_url = client.get_metadata_supplementary_download(&metadata.id, &supplementary_id).await?;
            if download_url.is_none() {
                return Err(Error::new(format!("missing supplementary: {}", supplementary_id)))
            }
            path = download_supplementary_path(&metadata.id, download_url.as_ref().unwrap()).await?;
        } else if metadata.content.type_ == "text/plain" {
            let download_url = client.get_metadata_download_url(&metadata.id).await?;
            path = download_path(&metadata.id, &download_url).await?;
        } else {
            let text_key = "text".to_string();
            let download_url = client.get_metadata_supplementary_download(&metadata.id, &text_key).await?;
            if download_url.is_none() {
                return Err(Error::new(format!("missing supplementary: {}", text_key)))
            }
            path = download_supplementary_path(&metadata.id, download_url.as_ref().unwrap()).await?;
        }
        let contents: String;
        if !path.is_empty() {
            context.add_file_clean(&path);
            let mut file = File::open(path).await?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await?;
            contents = from_utf8(&buffer).unwrap().to_string();
        } else {
            contents = String::new();
        }
        let storage_system_id = job.storage_systems.first().unwrap().system.id.clone();
        let documents = vec![SearchDocumentInput{ metadata_id: Some(metadata.id.clone()), collection_id: None, content: contents }];
        client.add_search_documents(&storage_system_id, documents).await?;
        Ok(())
    }
}
