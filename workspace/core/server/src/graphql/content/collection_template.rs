use crate::context::BoscaContext;
use crate::graphql::content::collection_attribute_object::CollectionTemplateAttributeObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::collection_template::CollectionTemplate;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::models::content::find_query::FindQueries;

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

    pub async fn collection_filter(&self) -> &Option<FindQueries> {
        &self.template.collection_filter
    }

    pub async fn metadata_filter(&self) -> &Option<FindQueries> {
        &self.template.metadata_filter
    }

    pub async fn attributes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CollectionTemplateAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collection_templates
            .get_template_attributes(&self.template.metadata_id, self.template.version)
            .await?
            .into_iter()
            .map(|a| {
                CollectionTemplateAttributeObject::new(
                    self.template.metadata_id,
                    self.template.version,
                    a,
                )
            })
            .collect())
    }
}
