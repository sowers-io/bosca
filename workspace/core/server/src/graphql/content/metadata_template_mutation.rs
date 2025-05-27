use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::document_template_container::DocumentTemplateContainerInput;
use crate::models::content::metadata::Metadata;
use crate::models::content::template_attribute::TemplateAttributeInput;
use async_graphql::*;

pub struct MetadataTemplateMutationObject {
    metadata: Metadata,
}

impl MetadataTemplateMutationObject {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

#[Object(name = "MetadataTemplateMutation")]
impl MetadataTemplateMutationObject {
    async fn set_default_attributes(
        &self,
        ctx: &Context<'_>,
        attributes: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .documents
            .set_default_attributes(&self.metadata.id, self.metadata.version, &attributes)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_content(
        &self,
        ctx: &Context<'_>,
        content: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .documents
            .set_content(&self.metadata.id, self.metadata.version, &content)
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
            .documents
            .set_configuration(&self.metadata.id, self.metadata.version, &configuration)
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
            .documents
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
            .documents
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
            .documents
            .delete_template_attribute(&self.metadata.id, self.metadata.version, &key)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_containers(
        &self,
        ctx: &Context<'_>,
        containers: Vec<DocumentTemplateContainerInput>,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .documents
            .set_template_containers(&self.metadata.id, self.metadata.version, &containers)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn add_container(
        &self,
        ctx: &Context<'_>,
        sort: i32,
        container: DocumentTemplateContainerInput,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .documents
            .add_template_container(&self.metadata.id, self.metadata.version, sort, &container)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn delete_container(
        &self,
        ctx: &Context<'_>,
        container_id: String,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .documents
            .delete_template_container(&self.metadata.id, self.metadata.version, &container_id)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }
}
