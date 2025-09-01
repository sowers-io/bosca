use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::guide_step_module::GuideStepModuleObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_step::GuideStep;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};

pub struct GuideStepObject {
    pub step: GuideStep,
    pub date: Option<DateTime<Utc>>,
}

impl GuideStepObject {
    pub fn new(step: GuideStep, date: Option<DateTime<Utc>>) -> Self {
        Self { step, date }
    }
}

#[Object(name = "GuideStep")]
impl GuideStepObject {
    pub async fn id(&self) -> i64 {
        self.step.id
    }

    pub async fn date(&self) -> Option<DateTime<Utc>> {
        self.date
    }

    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_id_with_version_advertised(
            self.step.step_metadata_id,
            self.step.step_metadata_version,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(Some(MetadataObject::new(metadata)))
    }

    pub async fn modules(&self, ctx: &Context<'_>) -> Result<Vec<GuideStepModuleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_id_with_version(
            self.step.step_metadata_id,
            self.step.step_metadata_version,
            PermissionAction::View,
        );
        ctx.metadata_permission_check(check).await?;
        let modules = ctx
            .content
            .guides
            .get_guide_step_modules(
                &self.step.metadata_id,
                self.step.metadata_version,
                self.step.id,
            )
            .await?;
        Ok(modules
            .into_iter()
            .map(GuideStepModuleObject::new)
            .collect())
    }
}
