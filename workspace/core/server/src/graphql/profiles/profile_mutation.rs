use crate::context::BoscaContext;
use crate::graphql::profiles::profile_guide_progress::ProfileGuideProgressObject;
use crate::models::profiles::profile::Profile;
use async_graphql::*;
use uuid::Uuid;

pub struct ProfileMutationObject {
    profile: Profile,
}

impl ProfileMutationObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
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
    ) -> Result<Option<ProfileGuideProgressObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Err(Error::new("Unauthorized"));
        }
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.profile
            .add_progress(
                ctx,
                &self.profile.id,
                &metadata_id,
                metadata_version,
                &attributes,
                step_id,
            )
            .await?;
        let progress = ctx
            .profile
            .get_progress(&self.profile.id, &metadata_id, metadata_version)
            .await?;
        Ok(progress.map(ProfileGuideProgressObject::new))
    }

    async fn add_bookmark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        version: Option<i32>,
        collection_id: Option<String>,
        attributes: Option<serde_json::Value>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Err(Error::new("Unauthorized"));
        }
        if let Some(metadata_id) = metadata_id {
            let metadata_id = Uuid::parse_str(&metadata_id)?;
            ctx.profile_bookmarks
                .add(ctx, &self.profile.id, Some(metadata_id), version, None, attributes)
                .await?;
            Ok(true)
        } else if let Some(collection_id) = collection_id {
            let collection_id = Uuid::parse_str(&collection_id)?;
            ctx.profile_bookmarks
                .add(ctx, &self.profile.id, None, None, Some(collection_id), attributes)
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_bookmark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        version: Option<i32>,
        collection_id: Option<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Err(Error::new("Unauthorized"));
        }
        if let Some(metadata_id) = metadata_id {
            let metadata_id = Uuid::parse_str(&metadata_id)?;
            ctx.profile_bookmarks
                .delete(ctx, &self.profile.id, Some(metadata_id), version, None)
                .await?;
            Ok(true)
        } else if let Some(collection_id) = collection_id {
            let collection_id = Uuid::parse_str(&collection_id)?;
            ctx.profile_bookmarks
                .delete(ctx, &self.profile.id, None, None, Some(collection_id))
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn add_mark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        version: Option<i32>,
        collection_id: Option<String>,
        attributes: Option<serde_json::Value>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Err(Error::new("Unauthorized"));
        }
        if let Some(metadata_id) = metadata_id {
            let metadata_id = Uuid::parse_str(&metadata_id)?;
            ctx.profile_marks
                .add(ctx, &self.profile.id, Some(metadata_id), version, None, attributes)
                .await?;
            Ok(true)
        } else if let Some(collection_id) = collection_id {
            let collection_id = Uuid::parse_str(&collection_id)?;
            ctx.profile_marks
                .add(ctx, &self.profile.id, None, None, Some(collection_id), attributes)
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_mark(
        &self,
        ctx: &Context<'_>,
        id: i64
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Err(Error::new("Unauthorized"));
        }
        ctx.profile_marks
            .delete(ctx, &self.profile.id, id)
            .await?;
        Ok(true)
    }

    async fn delete_attribute(
        &self,
        ctx: &Context<'_>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            ctx.check_has_service_account().await?;
        }
        let attribute_id = Uuid::parse_str(&attribute_id)?;
        let attributes = ctx.profile.get_attributes(&self.profile.id).await?;
        if let Some(attribute) = attributes.iter().find(|a| a.id == attribute_id) {
            let attribute_types = ctx.profile.get_attribute_types().await?;
            if let Some(attribute_type) = attribute_types.iter().find(|t| t.id == attribute.type_id) {
                if attribute_type.protected {
                    ctx.check_has_service_account().await?;
                }
            }
            ctx.profile
                .delete_profile_attribute(&self.profile.id, &attribute_id)
                .await?;
        }
        Ok(true)
    }
}
