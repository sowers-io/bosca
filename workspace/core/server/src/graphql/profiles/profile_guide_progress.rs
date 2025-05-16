use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_progress::GuideProgress;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct ProfileGuideProgressObject {
    progress: GuideProgress,
}

impl ProfileGuideProgressObject {
    pub fn new(progress: GuideProgress) -> Self {
        Self { progress }
    }
}

#[Object(name = "ProfileGuideProgress")]
impl ProfileGuideProgressObject {
    async fn metadata(&self, ctx: &Context<'_>) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx.check_metadata_version_action(&self.progress.metadata_id, self.progress.version, PermissionAction::View).await?;
        Ok(MetadataObject::new(metadata))
    }

    async fn percentage(&self, ctx: &Context<'_>) -> Result<f64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let count = ctx.content.guides.get_guide_step_count(&self.progress.metadata_id, self.progress.version).await?;
        if count == 0 {
            return Ok(0.0);
        }
        let done = self.progress.completed_step_ids.len() as f64;
        Ok((done / (count as f64)) * 100.0)
    }

    async fn attributes(&self) -> &Value {
        &self.progress.attributes
    }

    async fn started(&self) -> &DateTime<Utc> {
        &self.progress.started
    }

    async fn modified(&self) -> &DateTime<Utc> {
        &self.progress.modified
    }

    async fn completed(&self) -> &Option<DateTime<Utc>> {
        &self.progress.completed
    }

    async fn completed_step_ids(&self) -> &Vec<i64> {
        &self.progress.completed_step_ids
    }
}
