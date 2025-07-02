use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::metadata::Metadata;
use crate::models::content::template_attribute::TemplateAttributeInput;
use async_graphql::*;

pub struct GuideTemplateMutationObject {
    metadata: Metadata,
}

impl GuideTemplateMutationObject {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

#[Object(name = "GuideTemplateMutation")]
impl GuideTemplateMutationObject {
    async fn set_default_attributes(
        &self,
        ctx: &Context<'_>,
        attributes: serde_json::Value,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
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
            .guides
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
            .guides
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
            .guides
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
            .guides
            .delete_template_attribute(&self.metadata.id, self.metadata.version, &key)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_type(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "guideType")] guide_type: String,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .set_template_type(&self.metadata.id, self.metadata.version, &guide_type)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn set_rrule(
        &self,
        ctx: &Context<'_>,
        rrule: String,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .set_template_rrule(&self.metadata.id, self.metadata.version, &rrule)
            .await?;
        Ok(Some(MetadataObject::new(self.metadata.clone())))
    }

    async fn add_step(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepMetadataId")] step_metadata_id: String,
        #[graphql(name = "stepMetadataVersion")] step_metadata_version: Option<i32>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .add_template_step(&self.metadata.id, self.metadata.version, &step_metadata_id, step_metadata_version)
            .await?;
        Ok(true)
    }

    async fn remove_step(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepId")] step_id: i64,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .remove_template_step(&self.metadata.id, self.metadata.version, step_id)
            .await?;
        Ok(true)
    }

    async fn reorder_steps(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepIds")] step_ids: Vec<i64>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .reorder_template_steps(&self.metadata.id, self.metadata.version, &step_ids)
            .await?;
        Ok(true)
    }

    async fn add_module(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepId")] step_id: i64,
        #[graphql(name = "moduleMetadataId")] module_metadata_id: String,
        #[graphql(name = "moduleMetadataVersion")] module_metadata_version: i32,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let module_metadata_id = uuid::Uuid::parse_str(&module_metadata_id)?;
        ctx.content
            .guides
            .add_template_step_module(
                ctx,
                &self.metadata.id,
                self.metadata.version,
                step_id,
                &module_metadata_id,
                module_metadata_version,
            )
            .await?;
        Ok(true)
    }

    async fn remove_module(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepId")] step_id: i64,
        #[graphql(name = "moduleId")] module_id: i64,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .remove_template_step_module(
                ctx,
                &self.metadata.id,
                self.metadata.version,
                step_id,
                module_id,
            )
            .await?;
        Ok(true)
    }

    async fn reorder_modules(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "stepId")] step_id: i64,
        #[graphql(name = "moduleIds")] module_ids: Vec<i64>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content
            .guides
            .reorder_template_step_modules(
                ctx,
                &self.metadata.id,
                self.metadata.version,
                step_id,
                &module_ids,
            )
            .await?;
        Ok(true)
    }
}