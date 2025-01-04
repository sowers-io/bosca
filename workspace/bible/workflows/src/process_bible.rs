use async_trait::async_trait;
use bosca_bible_processor::process_path;
use serde_json::{Map, Value};
use bosca_client::client::add_collection::{CollectionInput, CollectionType};
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::enqueue_child_workflow::WorkflowConfigurationInput;
use bosca_client::client::find_collection::FindAttributeInput;
use bosca_client::download::download_path;
use bosca_workflows::activity::{Activity, ActivityContext, Error};
use crate::collections::new_bible_collection;

pub struct ProcessBibleActivity {
    id: String,
}

impl Default for ProcessBibleActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessBibleActivity {
    pub fn new() -> ProcessBibleActivity {
        ProcessBibleActivity {
            id: "bible.usx.process".to_string(),
        }
    }
}

impl ProcessBibleActivity {
    async fn find_root_bible_collection(&self, client: &Client) -> Result<Option<String>, Error> {
        let collections = client
            .find_collection(vec![
                FindAttributeInput {
                    key: "collection".to_string(),
                    value: "bible".to_string(),
                },
            ])
            .await?;
        Ok(collections.first().map(|c| c.id.clone()))
    }

    async fn get_root_bible_collection(
        &self,
        client: &Client,
    ) -> Result<String, Error> {
        if let Some(id) = self.find_root_bible_collection(client).await? {
            return Ok(id)
        }
        let mut attributes = Map::new();
        attributes.insert("collection".to_string(), Value::String("bible".to_string()));
        let response = client
            .add_collection(CollectionInput {
                parent_collection_id: Some("00000000-0000-0000-0000-000000000000".to_owned()),
                collection_type: Some(CollectionType::FOLDER),
                name: "Bibles".to_owned(),
                description: None,
                labels: None,
                attributes: Some(Value::Object(attributes)),
                state: None,
                index: None,
                ordering: None,
                collections: None,
                metadata: None,
            })
            .await?;
        Ok(response.add.id)
    }
}

#[async_trait]
impl Activity for ProcessBibleActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        context: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let source = client.get_source("workflow").await?.unwrap();
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let download_url = client.get_metadata_download_url(metadata_id).await?;
        let download = download_path(metadata_id, &download_url).await?;
        context.add_file_clean(&download);
        let bible = match process_path(download.as_str()) {
            Ok(bible) => bible,
            Err(e) => return Err(Error::new(e.to_string())),
        };
        let root_bibles_collection = self.get_root_bible_collection(client).await?;
        let bible_collection = new_bible_collection(&bible, metadata_id, &root_bibles_collection, &source);
        let result = client.add_collection(bible_collection).await?;
        let mut configuration = serde_json::map::Map::new();
        configuration.insert("recursive".to_owned(), Value::Bool(true));
        configuration.insert("collection_id".to_owned(), Value::String(result.add.id.to_owned()));
        client
            .enqueue_child_workflow(
                job.id.id,
                &job.id.queue,
                "collection.set.ready",
                vec![WorkflowConfigurationInput {
                    activity_id: "collection.set.ready".to_owned(),
                    configuration: Value::Object(configuration),
                }],
            )
            .await?;
        Ok(())
    }
}
