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

    pub async fn step(
        &self,
        ctx: &Context<'_>,
        step_id: i64,
    ) -> Result<Option<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let guide = ctx
            .content
            .guides
            .get_guide_step(&self.guide.metadata_id, self.guide.version, step_id)
            .await?;
        Ok(guide.map(GuideStepObject::new))
    }

    pub async fn step_count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx
            .content
            .guides
            .get_guide_step_count(&self.guide.metadata_id, self.guide.version)
            .await
    }

    pub async fn steps(
        &self,
        ctx: &Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let steps = ctx
            .content
            .guides
            .get_guide_steps(&self.guide.metadata_id, self.guide.version, offset, limit)
            .await?;
        Ok(steps.into_iter().map(GuideStepObject::new).collect())
    }
}
