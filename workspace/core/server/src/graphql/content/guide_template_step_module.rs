use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;
use crate::models::security::permission::PermissionAction;
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

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .check_metadata_version_action(
                &self.module.template_metadata_id,
                self.module.template_metadata_version,
                PermissionAction::View,
            )
            .await?;
        Ok(Some(MetadataObject::new(metadata)))
    }
}
