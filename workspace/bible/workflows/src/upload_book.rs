use async_trait::async_trait;
use bosca_workflows::activity::{Activity, ActivityContext, Error};
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::upload::upload_multipart_bytes;
use bytes::Bytes;
use serde_json::Value;
use bosca_client::client::add_activity::ActivityInput;
use crate::util::get_bible;

pub struct BookUpload {
    id: String,
}

impl Default for BookUpload {
    fn default() -> Self {
        Self::new()
    }
}

impl BookUpload {
    pub fn new() -> BookUpload {
        BookUpload {
            id: "bible.usx.book.upload".to_string(),
        }
    }
}

#[async_trait]
impl Activity for BookUpload {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Upload Book USX".to_string(),
            description: "Upload the Book USX as Metadata Content".to_string(),
            child_workflow_id: None,
            configuration: Value::Null,
            inputs: vec![],
            outputs: vec![],
        }
    }

    async fn execute(
        &self,
        client: &Client,
        context: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let book = {
            let bible = get_bible(client, context, job).await?;
            let book_usfm = job.metadata.as_ref().unwrap().attributes.get("bible.book.usfm").unwrap().as_str().unwrap().to_owned();
            let book = bible.books_by_usfm.get(&book_usfm).unwrap();
            let book = book.lock().unwrap();
            book.raw.to_owned()
        };
        let bytes = Bytes::from(book);
        let upload_url = client.get_metadata_upload_url(metadata_id).await?;
        upload_multipart_bytes(metadata_id, "bible/usx-book", &upload_url, bytes).await?;
        Ok(())
    }
}
