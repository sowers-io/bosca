use crate::context::BoscaContext;
use crate::graphql::content::guide_step::GuideStepObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide::Guide;
use crate::models::content::guide_type::GuideType;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use std::sync::atomic::AtomicI64;

pub struct GuideObject {
    pub guide: Guide,
    size: AtomicI64,
}

impl GuideObject {
    pub fn new(guide: Guide) -> Self {
        Self {
            guide,
            size: AtomicI64::new(-1),
        }
    }

    async fn get_size(&self, ctx: &BoscaContext) -> Result<i64, Error> {
        let mut size = self.size.load(std::sync::atomic::Ordering::Relaxed);
        if size == -1 {
            size = ctx
                .content
                .guides
                .get_guide_step_count(&self.guide.metadata_id, self.guide.version)
                .await?;
            self.size.store(size, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(size)
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

    pub async fn recurrences(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Vec<DateTime<Utc>>>, Error> {
        if let Some(rrule) = self.guide.rrule.clone() {
            let ctx = ctx.data::<BoscaContext>()?;
            let limit = self.get_size(ctx).await?;
            let recurrences = rrule.all(limit as u16);
            return Ok(Some(
                recurrences.dates.into_iter().map(|d| d.to_utc()).collect(),
            ));
        }
        Ok(None)
    }

    #[graphql(name = "type")]
    pub async fn guide_type(&self) -> GuideType {
        self.guide.guide_type
    }

    pub async fn step(
        &self,
        ctx: &Context<'_>,
        step_id: Option<i64>,
        date: Option<DateTime<Utc>>,
    ) -> Result<Option<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let step = if let Some(step_id) = step_id {
            ctx.content
                .guides
                .get_guide_step(&self.guide.metadata_id, self.guide.version, step_id)
                .await?
        } else if let Some(date) = date {
            if let Some(rrule) = self.guide.rrule.clone() {
                // TODO: cache this somewhere
                let size = self.get_size(ctx).await?;
                let recurrences = rrule.all(size as u16);
                let mut offset: i64 = 0;
                for d in recurrences.dates.into_iter().map(|d| d.to_utc()) {
                    if d >= date {
                        break;
                    }
                    offset += 1;
                }
                ctx.content
                    .guides
                    .get_guide_steps(
                        &self.guide.metadata_id,
                        self.guide.version,
                        Some(offset),
                        Some(1),
                    )
                    .await?
                    .into_iter()
                    .next()
            } else {
                return Err(Error::new("rrule not available"));
            }
        } else {
            return Err(Error::new("must provide either step_id or date"));
        };
        if let Some(step) = step {
            // TODO: cache this somewhere
            Ok(Some(if let Some(rrule) = self.guide.rrule.clone() {
                let recurrences = rrule.all((step.sort + 1) as u16);
                let mut dates = recurrences.dates.iter().map(|d| d.to_utc());
                for _ in 0..step.sort {
                    dates.next();
                }
                GuideStepObject::new(step, dates.next())
            } else {
                GuideStepObject::new(step, None)
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn step_count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        self.get_size(ctx).await
    }

    pub async fn step_by_offset(&self, ctx: &Context<'_>, offset: i64) -> Result<Option<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let steps = get_steps(ctx, &self.guide, Some(offset), Some(1)).await?;
        Ok(steps.into_iter().next())
    }

    pub async fn steps(
        &self,
        ctx: &Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<GuideStepObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        get_steps(ctx, &self.guide, offset, limit).await
    }
}

async fn get_steps(ctx: &BoscaContext, guide: &Guide, offset: Option<i64>, limit: Option<i64>) -> Result<Vec<GuideStepObject>, Error> {
    let steps = ctx
        .content
        .guides
        .get_guide_steps(&guide.metadata_id, guide.version, offset, limit)
        .await?;
    if let Some(rrule) = guide.rrule.clone() {
        if let Some(last_step) = steps.last() {
            // TODO: cache this somewhere
            let recurrences = rrule.all((last_step.sort + 1) as u16);
            let mut dates = recurrences.dates.into_iter().map(|d| d.to_utc());
            if let Some(offset) = offset {
                let mut x = 0;
                while x < offset {
                    dates.next();
                    x += 1;
                }
            }
            let mut results = Vec::new();
            for step in steps {
                results.push(GuideStepObject::new(step, dates.next()));
            }
            Ok(results)
        } else {
            Ok(Vec::new())
        }
    } else {
        Ok(steps
            .into_iter()
            .map(|s| GuideStepObject::new(s, None))
            .collect())
    }
}
