use crate::context::BoscaContext;
use crate::graphql::content::guide_step_module::GuideStepModuleObject;
use crate::models::content::guide_step::GuideStep;
use async_graphql::{Context, Error, Object};

pub struct GuideStepObject {
    pub step: GuideStep,
}

impl GuideStepObject {
    pub fn new(step: GuideStep) -> Self {
        Self { step }
    }
}

#[Object(name = "GuideStep")]
impl GuideStepObject {
    pub async fn modules(&self, ctx: &Context<'_>) -> Result<Vec<GuideStepModuleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let modules = ctx
            .content
            .guides
            .get_guide_step_modules(&self.step.metadata_id, self.step.version, self.step.id)
            .await?;
        Ok(modules.into_iter().map(GuideStepModuleObject::new).collect())
    }
}
