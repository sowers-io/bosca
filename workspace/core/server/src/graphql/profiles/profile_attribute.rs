use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::profiles::profile_attribute::ProfileAttribute;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;

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
            let check = PermissionCheck::new_with_metadata_id(metadata_id, PermissionAction::View);
            if let Ok(metadata) = ctx.metadata_permission_check(check).await {
                Ok(Some(MetadataObject::new(metadata)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
