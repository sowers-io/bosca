use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::profiles::profile_attribute::ProfileAttribute;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;

pub struct ProfileAttributeObject {
    attribute: ProfileAttribute,
}

impl ProfileAttributeObject {
    pub fn new(attribute: ProfileAttribute) -> Self {
        Self { attribute }
    }
}

#[Object(name = "ProfileAttribute")]
impl ProfileAttributeObject {
    async fn id(&self) -> String {
        self.attribute.id.to_string()
    }
    async fn type_id(&self) -> &String {
        &self.attribute.type_id
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.attribute.visibility
    }

    async fn confidence(&self) -> i32 {
        self.attribute.confidence
    }
    async fn priority(&self) -> i32 {
        self.attribute.priority
    }
    async fn source(&self) -> &String {
        &self.attribute.source
    }

    async fn attributes(&self) -> &Option<Value> {
        &self.attribute.attributes
    }
    async fn expires(&self) -> &Option<DateTime<Utc>> {
        &self.attribute.expiration
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = self.attribute.metadata_id {
            if let Ok(metadata) = ctx.check_metadata_action(&metadata_id, PermissionAction::View).await {
                Ok(Some(MetadataObject::new(metadata)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
