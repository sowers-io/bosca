use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::guide_template_step_module::GuideTemplateStepModuleObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_template_step::GuideTemplateStep;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct GuideTemplateStepObject {
    pub metadata_id: Uuid,
    pub metadata_version: i32,
    pub step: GuideTemplateStep,
}

impl GuideTemplateStepObject {
    pub fn new(metadata_id: Uuid, metadata_version: i32, step: GuideTemplateStep) -> Self {
        Self {
            metadata_id,
            metadata_version,
            step,
        }
    }
}

#[Object(name = "GuideTemplateStep")]
impl GuideTemplateStepObject {
    pub async fn id(&self) -> i64 {
        self.step.id
    }

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_id_with_version(
            self.step.template_metadata_id,
            self.step.template_metadata_version,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(Some(MetadataObject::new(metadata)))
    }

    pub async fn modules(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GuideTemplateStepModuleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let modules = ctx
            .content
            .guides
            .get_template_step_modules(&self.metadata_id, self.metadata_version, self.step.id)
            .await?;
        Ok(modules
            .into_iter()
            .map(GuideTemplateStepModuleObject::new)
            .collect())
    }
}
