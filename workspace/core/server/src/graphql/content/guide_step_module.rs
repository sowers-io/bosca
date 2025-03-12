use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_step_module::GuideStepModule;
use crate::models::security::permission::PermissionAction;
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
    pub async fn id(&self) -> i64 {
        self.module.id
    }

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .check_metadata_version_action(
                &self.module.module_metadata_id,
                self.module.module_metadata_version,
                PermissionAction::View,
            )
            .await?;
        Ok(Some(MetadataObject::new(metadata)))
    }
}
