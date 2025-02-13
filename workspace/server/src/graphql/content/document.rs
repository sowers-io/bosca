use crate::context::BoscaContext;
use crate::graphql::content::document_block::DocumentBlockObject;
use crate::models::content::document::Document;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct DocumentObject {
    pub metadata_id: Uuid,
    pub version: i32,
    pub document: Document,
}

impl DocumentObject {
    pub fn new(metadata_id: Uuid, version: i32, document: Document) -> Self {
        Self {
            metadata_id,
            version,
            document,
        }
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

    pub async fn allow_user_defined_blocks(&self) -> bool {
        self.document.allow_user_defined_blocks
    }

    pub async fn blocks(&self, ctx: &Context<'_>) -> Result<Vec<DocumentBlockObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_blocks(&self.metadata_id, self.version)
            .await?
            .into_iter()
            .map(|d| DocumentBlockObject::new(self.metadata_id, self.version, d))
            .collect())
    }
}
