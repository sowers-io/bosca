use crate::models::profile::profile_visibility::ProfileVisibility;
use crate::models::profile::profile_attribute::ProfileAttribute;
use async_graphql::Object;
use chrono::{DateTime, Utc};

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

    async fn attributes(&self) -> &serde_json::Value {
        &self.attribute.attributes
    }
    async fn expires(&self) -> &Option<DateTime<Utc>> {
        &self.attribute.expiration
    }
}
