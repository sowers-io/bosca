use crate::models::content::document::Document;
use async_graphql::Object;
use serde_json::Value;

pub struct DocumentObject {
    pub document: Document,
}

impl DocumentObject {
    pub fn new(document: Document) -> Self {
        Self { document }
    }
}

#[Object(name = "Document")]
impl DocumentObject {
    pub async fn template_metadata_id(&self) -> Option<String> {
        self.document.template_metadata_id.map(|id| id.to_string())
    }

    pub async fn template_metadata_version(&self) -> Option<i32> {
        self.document.template_metadata_version
    }

    pub async fn title(&self) -> &String {
        &self.document.title
    }

    pub async fn content(&self) -> &Value {
        &self.document.content
    }
}
