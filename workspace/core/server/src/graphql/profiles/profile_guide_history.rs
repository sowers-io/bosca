use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::models::content::guide_history::GuideHistory;

pub struct ProfileGuideHistoryObject {
    history: GuideHistory,
}

impl ProfileGuideHistoryObject {
    pub fn new(history: GuideHistory) -> Self {
        Self { history }
    }
}

#[Object(name = "ProfileGuideHistory")]
impl ProfileGuideHistoryObject {
    async fn metadata(&self, ctx: &Context<'_>) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx.check_metadata_version_action(&self.history.metadata_id, self.history.version, PermissionAction::View).await?;
        Ok(MetadataObject::new(metadata))
    }

    async fn attributes(&self) -> &Value {
        &self.history.attributes
    }

    async fn completed(&self) -> &Option<DateTime<Utc>> {
        &self.history.completed
    }
}
