use crate::context::BoscaContext;
use crate::graphql::content::guide_template_step::GuideTemplateStepObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::template_attribute::TemplateAttributeObject;
use crate::models::content::guide_template::GuideTemplate;
use crate::models::content::guide_type::GuideType;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct GuideTemplateObject {
    pub template: GuideTemplate,
}

impl GuideTemplateObject {
    pub fn new(template: GuideTemplate) -> Self {
        Self { template }
    }
}

#[Object(name = "GuideTemplate")]
impl GuideTemplateObject {
    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .content
            .metadata
            .get_by_version(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(metadata.map(MetadataObject::new))
    }

    pub async fn rrule(&self) -> Option<String> {
        self.template.rrule.as_ref().map(|r| r.to_string())
    }

    #[graphql(name = "type")]
    pub async fn guide_type(&self) -> GuideType {
        self.template.guide_type
    }

    pub async fn default_attributes(&self) -> &Option<Value> {
        &self.template.default_attributes
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
            .get_template_attributes(&self.template.metadata_id, self.template.version)
            .await?
        {
            let workflows = ctx
                .content
                .guides
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

    pub async fn steps(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GuideTemplateStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let steps = ctx
            .content
            .guides
            .get_template_steps(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(steps
            .into_iter()
            .map(GuideTemplateStepObject::new)
            .collect())
    }
}
