use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::template_attribute::TemplateAttributeObject;
use crate::models::content::collection_template::{CollectionTemplate, CollectionTemplateFilters};
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::models::content::ordering::Ordering;

pub struct CollectionTemplateObject {
    pub template: CollectionTemplate,
}

impl CollectionTemplateObject {
    pub fn new(template: CollectionTemplate) -> Self {
        Self { template }
    }
}

#[Object(name = "CollectionTemplate")]
impl CollectionTemplateObject {

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .content
            .metadata
            .get_by_version(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(metadata.map(MetadataObject::new))
    }

    pub async fn default_attributes(&self) -> &Option<Value> {
        &self.template.default_attributes
    }

    pub async fn configuration(&self) -> &Option<Value> {
        &self.template.configuration
    }

    pub async fn attributes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TemplateAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut attributes = Vec::new();
        for attribute in ctx
            .content
            .collection_templates
            .get_template_attributes(&self.template.metadata_id, self.template.version)
            .await?
        {
            let workflows = ctx
                .content
                .collection_templates
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

    pub async fn ordering(&self) -> &Option<Vec<Ordering>> {
        &self.template.ordering
    }

    pub async fn filters(&self) -> &Option<CollectionTemplateFilters> {
        &self.template.filters
    }

}
