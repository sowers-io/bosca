use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;
use async_graphql::{Context, Error, Object};

pub struct GuideTemplateStepModuleObject {
    pub module: GuideTemplateStepModule,
}

impl GuideTemplateStepModuleObject {
    pub fn new(module: GuideTemplateStepModule) -> Self {
        Self { module }
    }
}

#[Object(name = "GuideTemplateStepModule")]
impl GuideTemplateStepModuleObject {

    pub async fn id(&self) -> i64 {
        self.module.id
    }

    pub async fn metadata(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content.metadata.get_by_version(
            &self.module.template_metadata_id,
            self.module.template_metadata_version,
        ).await?.map(MetadataObject::new))
    }
}
