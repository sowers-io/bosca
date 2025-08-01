use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::document::Document;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
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
    pub async fn template(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = &self.document.template_metadata_id {
            if let Some(version) = &self.document.template_metadata_version {
                let check = PermissionCheck::new_with_metadata_id_with_version(
                    *id,
                    *version,
                    PermissionAction::View,
                );
                let metadata = ctx.metadata_permission_check(check).await?;
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
