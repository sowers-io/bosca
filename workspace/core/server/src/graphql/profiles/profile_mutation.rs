use crate::context::BoscaContext;
use crate::models::profiles::profile::Profile;
use async_graphql::*;
use uuid::Uuid;

pub struct ProfileMutationObject {
    profile: Profile,
}

impl ProfileMutationObject {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile
        }
    }
}

#[Object(name = "ProfileMutation")]
impl ProfileMutationObject {
    async fn add_progress(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
        attributes: serde_json::Value,
        step_id: i64,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        let result = ctx.profile.add_progress(ctx, &self.profile.id, &metadata_id, metadata_version, &attributes, step_id).await?;
        Ok(result)
    }

    async fn add_bookmark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        version: Option<i64>,
        collection_id: Option<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = metadata_id {
            let metadata_id = Uuid::parse_str(&metadata_id)?;
            ctx.profile.add_bookmark(ctx, &self.profile.id, Some(metadata_id), version, None).await?;
            Ok(true)
        } else if let Some(collection_id) = collection_id {
            let collection_id = Uuid::parse_str(&collection_id)?;
            ctx.profile.add_bookmark(ctx, &self.profile.id, None, None, Some(collection_id)).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_bookmark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        version: Option<i64>,
        collection_id: Option<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = metadata_id {
            let metadata_id = Uuid::parse_str(&metadata_id)?;
            ctx.profile.delete_bookmark(ctx, &self.profile.id, Some(metadata_id), version, None).await?;
            Ok(true)
        } else if let Some(collection_id) = collection_id {
            let collection_id = Uuid::parse_str(&collection_id)?;
            ctx.profile.delete_bookmark(ctx, &self.profile.id, None, None, Some(collection_id)).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_attribute(
        &self,
        ctx: &Context<'_>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let attribute_id = Uuid::parse_str(&attribute_id)?;
        ctx.profile.delete_profile_attribute(&self.profile.id, &attribute_id).await?;
        Ok(true)
    }
}
