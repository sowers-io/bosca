use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_progress::GuideProgress;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

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
        let check = PermissionCheck::new_with_metadata_id_with_version(
            self.progress.metadata_id,
            self.progress.version,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(MetadataObject::new(metadata))
    }

    async fn percentage(&self, ctx: &Context<'_>) -> Result<f64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let count = ctx
            .content
            .guides
            .get_guide_step_count(&self.progress.metadata_id, self.progress.version)
            .await?;
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

    async fn completed_step_ids(&self) -> &Vec<i64> {
        &self.progress.completed_step_ids
    }

    async fn next_step_id(&self, ctx: &Context<'_>) -> Result<Option<i64>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let all_step_ids = ctx
            .content
            .guides
            .get_guide_step_ids(&self.progress.metadata_id, self.progress.version)
            .await?;
        let mut completions = HashMap::new();
        for id in &self.progress.completed_step_ids {
            completions.insert(id, true);
        }
        let mut first_not_complete = None;
        let mut next_step_id = None;
        for step_id in all_step_ids {
            let session_complete = completions.get(&step_id).copied().unwrap_or(false);
            if !session_complete && first_not_complete.is_none() {
                first_not_complete = Some(step_id);
            }
            if !session_complete && next_step_id.is_none() {
                next_step_id = Some(step_id);
            } else if session_complete {
                next_step_id = None;
            }
        }
        Ok(if next_step_id.is_none() {
            first_not_complete
        } else {
            next_step_id
        })
    }
}
