use crate::context::BoscaContext;
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
        if let Some(id) = &self.step.template_metadata_id {
            if let Some(version) = &self.step.template_metadata_version {
                let metadata = ctx
                    .check_metadata_version_action(id, *version, PermissionAction::View)
                    .await?;
                return Ok(Some(MetadataObject::new(metadata)));
            }
        }
        Ok(None)
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
