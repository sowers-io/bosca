use crate::models::content::document::Document;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;

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
    pub async fn template(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = &self.document.template_metadata_id {
            if let Some(version) = &self.document.template_metadata_version {
                let metadata = ctx.check_metadata_version_action(
                    &id,
                    *version,
                    PermissionAction::View,
                )
                .await?;
                return Ok(Some(MetadataObject::new(metadata)));
            }
        }
        Ok(None)
    }

    pub async fn title(&self) -> &String {
        &self.document.title
    }

    pub async fn content(&self) -> &Value {
        &self.document.content
    }
}
