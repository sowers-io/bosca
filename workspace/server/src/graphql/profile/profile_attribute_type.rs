use crate::models::profile::profile_attribute_type::ProfileAttributeType;
use async_graphql::Object;
use crate::models::profile::profile_visibility::ProfileVisibility;

pub struct ProfileAttributeTypeObject {
    attribute_type: ProfileAttributeType,
}

impl ProfileAttributeTypeObject {
    pub fn new(attribute_type: ProfileAttributeType) -> Self {
        Self { attribute_type }
    }
}

#[Object(name = "ProfileAttributeType")]
impl ProfileAttributeTypeObject {
    async fn id(&self) -> &String {
        &self.attribute_type.id
    }

    async fn name(&self) -> &String {
        &self.attribute_type.name
    }

    async fn description(&self) -> &String {
        &self.attribute_type.description
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.attribute_type.visibility
    }
}

