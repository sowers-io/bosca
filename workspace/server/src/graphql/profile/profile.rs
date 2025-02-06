use async_graphql::Object;
use crate::models::profile::profile::{Profile, ProfileVisibility};

pub struct ProfileObject {
    profile: Profile,
}

impl ProfileObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "Profile")]
impl ProfileObject {
    async fn id(&self) -> String {
        self.profile.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.profile.name
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.profile.visibility
    }
}

