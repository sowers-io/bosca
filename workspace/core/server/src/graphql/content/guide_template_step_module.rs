use crate::context::{BoscaContext, PermissionCheck};
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
        let check = PermissionCheck::new_with_metadata_id_with_version(
            self.module.template_metadata_id,
            self.module.template_metadata_version,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(Some(MetadataObject::new(metadata)))
    }
}
