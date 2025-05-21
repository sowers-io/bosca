use crate::models::content::document_collaboration::DocumentCollaboration;
use async_graphql::Object;
use chrono::{DateTime, Utc};

pub struct DocumentCollaborationObject {
    pub document: DocumentCollaboration,
}

impl DocumentCollaborationObject {
    pub fn new(document: DocumentCollaboration) -> Self {
        Self { document }
    }
}

#[Object(name = "DocumentCollaboration")]
impl DocumentCollaborationObject {
    pub async fn content(&self) -> &Vec<u8> {
        &self.document.content
    }
    pub async fn created(&self) -> &DateTime<Utc> {
        &self.document.created
    }
    pub async fn modified(&self) -> &DateTime<Utc> {
        &self.document.modified
    }
}
