use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use bosca_bible_processor::usx::item::{IUsxItem, UsxItem};
use bosca_workflows::activity::{Activity, ActivityContext, Error};
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::upload::{upload_multipart_bytes, upload_multipart_supplementary_bytes};
use bytes::Bytes;
use serde_json::Value;
use bosca_bible_processor::html_context::HtmlContext;
use bosca_client::client::add_activity::ActivityInput;
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use crate::util::get_bible;

pub struct ChapterUpload {
    id: String,
}

impl Default for ChapterUpload {
    fn default() -> Self {
        Self::new()
    }
}

impl ChapterUpload {
    pub fn new() -> ChapterUpload {
        ChapterUpload {
            id: "bible.usx.chapter.upload".to_string(),
        }
    }
}

#[async_trait]
impl Activity for ChapterUpload {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Upload Chapter USX".to_string(),
            description: "Upload the Chapter USX as Metadata Content".to_string(),
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
        let (chapter, content) = {
            let bible = get_bible(client, context, job).await?;
            let book_usfm = job.metadata.as_ref().unwrap().attributes.get("bible.book.usfm").unwrap().as_str().unwrap().to_owned();
            let chapter_usfm = job.metadata.as_ref().unwrap().attributes.get("bible.chapter.usfm").unwrap().as_str().unwrap().to_owned();
            let book = bible.books_by_usfm.get(&book_usfm).unwrap();
            let book_guard = book.lock().unwrap();
            let chapter = book_guard.chapters_by_usfm.get(&chapter_usfm).unwrap();
            let chapter_guard = chapter.lock().unwrap();
            let c = match chapter_guard.deref() {
                UsxItem::Chapter(chapter) => chapter,
                _ => return Err(Error::new("couldn't find chapter".to_owned()))
            };
            let position = c.container.position.as_ref().unwrap();
            let content = book_guard.get_raw_content(position.lock().unwrap().deref()).to_owned();
            (Arc::clone(chapter), content)
        };
        let bytes = Bytes::from(content);
        let upload_url = client.get_metadata_upload_url(metadata_id).await?;
        upload_multipart_bytes(metadata_id, "bible/usx-chapter", &upload_url, bytes).await?;
        let mut ctx = HtmlContext::new(true, true, true, true);
        let html = chapter.lock().unwrap().to_html(&mut ctx);
        let mime_type = "text/html";
        let key = "html";
        let bytes = Bytes::from(html);
        if job.metadata.as_ref().unwrap().supplementary.is_empty() {
            client.add_metadata_supplementary(MetadataSupplementaryInput {
                metadata_id: metadata_id.to_owned(),
                key: key.to_owned(),
                attributes: None,
                name: "Verse Html".to_owned(),
                content_type: mime_type.to_owned(),
                content_length: None,
                source_id: None,
                source_identifier: None,
            }).await?;
        }
        let upload_url = client.get_metadata_supplementary_upload(metadata_id, key).await?;
        upload_multipart_supplementary_bytes(metadata_id, mime_type, &upload_url, bytes).await?;
        Ok(())
    }
}
