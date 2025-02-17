use crate::context::BoscaContext;
use crate::graphql::content::document_template_attribute_object::DocumentTemplateAttributeObject;
use crate::models::content::document_template::DocumentTemplate;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;

pub struct DocumentTemplateObject {
    pub metadata_id: Uuid,
    pub version: i32,
    pub template: DocumentTemplate,
}

impl DocumentTemplateObject {
    pub fn new(metadata_id: Uuid, version: i32, template: DocumentTemplate) -> Self {
        Self {
            metadata_id,
            version,
            template,
        }
    }
}

#[Object(name = "DocumentTemplate")]
impl DocumentTemplateObject {
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
            .get_template_attributes(&self.metadata_id, self.version)
            .await?
            .into_iter()
            .map(|a| DocumentTemplateAttributeObject::new(self.metadata_id, self.version, a))
            .collect())
    }
}
