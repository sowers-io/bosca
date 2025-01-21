use std::ops::Deref;
use async_trait::async_trait;
use bosca_bible_processor::usx::item::UsxItem;
use bosca_workflows::activity::{Activity, ActivityContext, Error};
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::upload::{upload_multipart_bytes, upload_multipart_supplementary_bytes};
use bytes::Bytes;
use serde_json::Value;
use bosca_bible_processor::string_context::StringContext;
use bosca_client::client::add_activity::ActivityInput;
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use crate::util::get_bible;

pub struct VerseUpload {
    id: String,
}

impl Default for VerseUpload {
    fn default() -> Self {
        Self::new()
    }
}

impl VerseUpload {
    pub fn new() -> VerseUpload {
        VerseUpload {
            id: "bible.usx.verse.upload".to_string(),
        }
    }
}

#[async_trait]
impl Activity for VerseUpload {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Upload Verse USX".to_string(),
            description: "Upload the Verse USX as Metadata Content".to_string(),
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
        let verse = {
            let bible = get_bible(client, context, job).await?;
            let book_usfm = job.metadata.as_ref().unwrap().attributes.get("bible.book.usfm").unwrap().as_str().unwrap().to_owned();
            let chapter_usfm = job.metadata.as_ref().unwrap().attributes.get("bible.chapter.usfm").unwrap().as_str().unwrap().to_owned();
            let usfm = job.metadata.as_ref().unwrap().attributes.get("bible.verse.usfm").unwrap().as_str().unwrap();
            let book = bible.books_by_usfm.get(&book_usfm).unwrap();
            let book = book.lock().unwrap();
            let chapter = book.chapters_by_usfm.get(&chapter_usfm).unwrap();
            let chapter = chapter.lock().unwrap();
            let chapter = match chapter.deref() {
                UsxItem::Chapter(chapter) => chapter,
                _ => return Err(Error::new("couldn't find chapter".to_owned()))
            };
            chapter.get_verse(book.deref(), usfm)
        };
        if let Some(item) = verse {
            let ctx = StringContext::new(false, false, false, false);
            let text = item.to_string(&Some(ctx));
            let text_bytes = Bytes::from(text);
            let bytes = Bytes::from(item.raw);
            let upload_url = client.get_metadata_upload_url(metadata_id).await?;
            upload_multipart_bytes(metadata_id, "bible/usx-verse", &upload_url, bytes).await?;
            let mime_type = "text/plain";
            let key = "text";
            if job.metadata.as_ref().unwrap().supplementary.is_empty() {
                client.add_metadata_supplementary(MetadataSupplementaryInput {
                    metadata_id: metadata_id.to_owned(),
                    key: key.to_owned(),
                    attributes: None,
                    name: "Verse Text".to_owned(),
                    content_type: mime_type.to_owned(),
                    content_length: None,
                    source_id: None,
                    source_identifier: None,
                }).await?;
            }
            let upload_url = client.get_metadata_supplementary_upload(metadata_id, key).await?;
            upload_multipart_supplementary_bytes(metadata_id, mime_type, &upload_url, text_bytes).await?;
        } else {
            return Err(Error::new("couldn't find verse".to_owned()));
        }
        Ok(())
    }
}
