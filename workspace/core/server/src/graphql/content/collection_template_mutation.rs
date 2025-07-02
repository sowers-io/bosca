use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::metadata::Metadata;
use crate::models::content::template_attribute::TemplateAttributeInput;
use async_graphql::*;

pub struct CollectionTemplateMutationObject {
    metadata: Metadata
}

impl CollectionTemplateMutationObject {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

#[Object(name = "CollectionTemplateMutation")]
impl CollectionTemplateMutationObject {

    async fn set_default_attributes(
        &self,
        ctx: &Context<'_>,
        attributes: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .set_default_attributes(&self.metadata.id, self.metadata.version, &attributes)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_configuration(
        &self,
        ctx: &Context<'_>,
        configuration: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .set_configuration(&self.metadata.id, self.metadata.version, &configuration)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_filters(
        &self,
        ctx: &Context<'_>,
        filters: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .set_filters(ctx, &self.metadata.id, self.metadata.version, &filters)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_ordering(
        &self,
        ctx: &Context<'_>,
        ordering: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .set_ordering(ctx, &self.metadata.id, self.metadata.version, &ordering)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_attributes(
        &self,
        ctx: &Context<'_>,
        attributes: Vec<TemplateAttributeInput>,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .set_template_attributes(ctx, &self.metadata.id, self.metadata.version, &attributes)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn add_attribute(
        &self,
        ctx: &Context<'_>,
        sort: i32,
        attribute: TemplateAttributeInput,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .add_template_attribute(&self.metadata.id, self.metadata.version, sort, &attribute)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn delete_attribute(
        &self,
        ctx: &Context<'_>,
        key: String,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .collection_templates
            .delete_template_attribute(&self.metadata.id, self.metadata.version, &key)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }
}
