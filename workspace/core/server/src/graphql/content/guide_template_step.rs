use crate::context::BoscaContext;
use crate::graphql::content::guide_template_step_module::GuideTemplateStepModuleObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::template_attribute_object::TemplateAttributeObject;
use crate::models::content::guide_template_step::GuideTemplateStep;
use async_graphql::{Context, Error, Object};

pub struct GuideTemplateStepObject {
    pub step: GuideTemplateStep,
}

impl GuideTemplateStepObject {
    pub fn new(step: GuideTemplateStep) -> Self {
        Self { step }
    }
}

#[Object(name = "GuideTemplateStep")]
impl GuideTemplateStepObject {
    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = &self.step.template_metadata_id {
            if let Some(version) = &self.step.template_metadata_version {
                let metadata = ctx.content.metadata.get_by_version(id, *version).await?;
                return Ok(metadata.map(MetadataObject::new));
            }
        }
        Ok(None)
    }

    pub async fn attributes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TemplateAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut attributes = Vec::new();
        for attribute in ctx
            .content
            .guides
            .get_template_step_attributes(&self.step.metadata_id, self.step.version, self.step.id)
            .await?
        {
            let workflows = ctx
                .content
                .guides
                .get_template_step_attribute_workflows(
                    &self.step.metadata_id,
                    self.step.version,
                    self.step.id,
                    &attribute.key,
                )
                .await?;
            attributes.push(TemplateAttributeObject::new(attribute, workflows));
        }
        Ok(attributes)
    }

    pub async fn modules(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GuideTemplateStepModuleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let modules = ctx
            .content
            .guides
            .get_template_step_modules(&self.step.metadata_id, self.step.version, self.step.id)
            .await?;
        Ok(modules
            .into_iter()
            .map(GuideTemplateStepModuleObject::new)
            .collect())
    }
}
