use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_step_module::GuideStepModule;
use async_graphql::{Context, Error, Object};

pub struct GuideStepModuleObject {
    pub module: GuideStepModule,
}

impl GuideStepModuleObject {
    pub fn new(module: GuideStepModule) -> Self {
        Self { module }
    }
}

#[Object(name = "GuideStepModule")]
impl GuideStepModuleObject {

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .content
            .metadata
            .get_by_version(&self.module.module_metadata_id, self.module.module_metadata_version)
            .await?;
        Ok(metadata.map(MetadataObject::new))
    }
}
