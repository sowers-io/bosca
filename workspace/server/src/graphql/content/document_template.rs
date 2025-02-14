use crate::context::BoscaContext;
use crate::graphql::content::document_template_attribute_object::DocumentTemplateAttributeObject;
use crate::graphql::content::document_template_block::DocumentTemplateBlockObject;
use crate::models::content::document_template::DocumentTemplate;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
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
    pub async fn id(&self) -> i64 {
        self.template.id
    }

    pub async fn name(&self) -> &String {
        &self.template.name
    }

    pub async fn description(&self) -> &String {
        &self.template.description
    }

    pub async fn configuration(&self) -> &Value {
        &self.template.configuration
    }

    pub async fn allow_user_defined_blocks(&self) -> bool {
        self.template.allow_user_defined_blocks
    }

    pub async fn created(&self) -> &DateTime<Utc> {
        &self.template.created
    }

    pub async fn modified(&self) -> &DateTime<Utc> {
        &self.template.modified
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

    pub async fn blocks(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DocumentTemplateBlockObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_template_blocks(&self.metadata_id, self.version)
            .await?
            .into_iter()
            .map(DocumentTemplateBlockObject::new)
            .collect())
    }
}
