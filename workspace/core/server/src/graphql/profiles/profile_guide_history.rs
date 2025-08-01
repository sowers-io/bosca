use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::guide_history::GuideHistory;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;

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
        let check = PermissionCheck::new_with_metadata_id_with_version(
            self.history.metadata_id,
            self.history.version,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(MetadataObject::new(metadata))
    }

    async fn attributes(&self) -> &Value {
        &self.history.attributes
    }

    async fn completed(&self) -> &Option<DateTime<Utc>> {
        &self.history.completed
    }
}
