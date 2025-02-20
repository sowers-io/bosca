use crate::context::BoscaContext;
use crate::graphql::content::document_template_attribute_object::DocumentTemplateAttributeObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::document_template::DocumentTemplate;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct DocumentTemplateObject {
    pub template: DocumentTemplate,
}

impl DocumentTemplateObject {
    pub fn new(template: DocumentTemplate) -> Self {
        Self { template }
    }
}

#[Object(name = "DocumentTemplate")]
impl DocumentTemplateObject {
    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .content
            .metadata
            .get_by_version(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(metadata.map(MetadataObject::new))
    }

    pub async fn configuration(&self) -> &Option<Value> {
        &self.template.configuration
    }

    pub async fn schema(&self) -> &Option<Value> {
        &self.template.schema
    }

    pub async fn content(&self) -> &Value {
        &self.template.content
    }

    pub async fn attributes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DocumentTemplateAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_template_attributes(&self.template.metadata_id, self.template.version)
            .await?
            .into_iter()
            .map(|a| {
                DocumentTemplateAttributeObject::new(
                    self.template.metadata_id,
                    self.template.version,
                    a,
                )
            })
            .collect())
    }
}
