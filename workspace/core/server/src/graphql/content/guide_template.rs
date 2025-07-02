use crate::context::BoscaContext;
use crate::graphql::content::guide_template_step::GuideTemplateStepObject;
use crate::graphql::content::template_attribute::TemplateAttributeObject;
use crate::models::content::guide_template::GuideTemplate;
use crate::models::content::guide_type::GuideType;
use async_graphql::{Context, Error, Object};

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
    pub async fn rrule(&self) -> Option<String> {
        self.template.rrule.as_ref().map(|r| r.to_string())
    }

    #[graphql(name = "type")]
    pub async fn guide_type(&self) -> GuideType {
        self.template.guide_type
    }

    pub async fn default_attributes(&self) -> Option<serde_json::Value> {
        self.template.default_attributes.clone()
    }

    pub async fn configuration(&self) -> Option<serde_json::Value> {
        self.template.configuration.clone()
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
            // For now, use empty workflows - we can implement guide template workflows later
            let workflows = Vec::new();
            attributes.push(TemplateAttributeObject::new(attribute, workflows));
        }
        Ok(attributes)
    }

    pub async fn steps(&self, ctx: &Context<'_>) -> Result<Vec<GuideTemplateStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let steps = ctx
            .content
            .guides
            .get_template_steps(&self.template.metadata_id, self.template.version)
            .await?;
        Ok(steps
            .into_iter()
            .map(|s| {
                GuideTemplateStepObject::new(self.template.metadata_id, self.template.version, s)
            })
            .collect())
    }
}
