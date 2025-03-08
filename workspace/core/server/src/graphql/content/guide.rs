use crate::context::BoscaContext;
use crate::graphql::content::guide_step::GuideStepObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide::Guide;
use crate::models::content::guide_type::GuideType;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};

pub struct GuideObject {
    pub guide: Guide,
}

impl GuideObject {
    pub fn new(guide: Guide) -> Self {
        Self { guide }
    }
}

#[Object(name = "Guide")]
impl GuideObject {
    pub async fn template(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = &self.guide.template_metadata_id {
            if let Some(version) = &self.guide.template_metadata_version {
                let metadata = ctx
                    .check_metadata_version_action(id, *version, PermissionAction::View)
                    .await?;
                return Ok(Some(MetadataObject::new(metadata)));
            }
        }
        Ok(None)
    }

    pub async fn rrule(&self) -> Option<String> {
        self.guide.rrule.as_ref().map(|rrule| rrule.to_string())
    }

    #[graphql(name = "type")]
    pub async fn guide_type(&self) -> GuideType {
        self.guide.guide_type
    }

    pub async fn steps(&self, ctx: &Context<'_>) -> Result<Vec<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let steps = ctx
            .content
            .guides
            .get_guide_steps(&self.guide.metadata_id, self.guide.version)
            .await?;
        Ok(steps.into_iter().map(GuideStepObject::new).collect())
    }
}
