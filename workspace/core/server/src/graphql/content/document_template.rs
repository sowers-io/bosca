use crate::context::BoscaContext;
use crate::graphql::content::document_template_container::DocumentTemplateContainerObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::template_attribute::TemplateAttributeObject;
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

    pub async fn default_attributes(&self) -> &Option<Value> {
        &self.template.default_attributes
    }

    pub async fn containers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DocumentTemplateContainerObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let containers = ctx
            .content
            .documents
            .get_template_containers(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(containers
            .into_iter()
            .map(|c| {
                DocumentTemplateContainerObject::new(
                    self.template.metadata_id,
                    self.template.version,
                    c,
                )
            })
            .collect())
    }

    pub async fn attributes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TemplateAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut attributes = Vec::new();
        for attribute in ctx
            .content
            .documents
            .get_template_attributes(&self.template.metadata_id, self.template.version)
            .await?
        {
            let workflows = ctx
                .content
                .documents
                .get_template_attribute_workflows(
                    &self.template.metadata_id,
                    self.template.version,
                    &attribute.key,
                )
                .await?;
            attributes.push(TemplateAttributeObject::new(attribute, workflows));
        }
        Ok(attributes)
    }
}
