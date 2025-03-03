use crate::context::BoscaContext;
use crate::graphql::content::guide_template_step_module::GuideTemplateStepModuleObject;
use crate::models::content::guide_template_step::GuideTemplateStep;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::graphql::content::template_attribute_object::TemplateAttributeObject;

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
    pub async fn name(&self) -> &String {
        &self.step.name
    }

    pub async fn description(&self) -> &String {
        &self.step.description
    }

    pub async fn configuration(&self) -> &Option<Value> {
        &self.step.configuration
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
        Ok(modules.into_iter().map(|m| GuideTemplateStepModuleObject::new(m)).collect())
    }
}
