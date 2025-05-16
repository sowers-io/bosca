use crate::models::profiles::profile::Profile;
use async_graphql::{Error, Object};
use crate::graphql::profiles::profile_guide_histories::ProfileGuideHistoriesObject;
use crate::graphql::profiles::profile_progressions::ProfileGuideProgressionsObject;

pub struct ProfileGuidesObject {
    profile: Profile,
}

impl ProfileGuidesObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "ProfileGuides")]
impl ProfileGuidesObject {

    async fn progressions(&self) -> Result<ProfileGuideProgressionsObject, Error> {
        Ok(ProfileGuideProgressionsObject::new(self.profile.clone()))
    }

    async fn history(&self) -> Result<ProfileGuideHistoriesObject, Error> {
        Ok(ProfileGuideHistoriesObject::new(self.profile.clone()))
    }
}
