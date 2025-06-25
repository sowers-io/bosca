use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_attribute_type::ProfileAttributeTypeInput;
use async_graphql::*;
use uuid::Uuid;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::profiles::profile_mutation::ProfileMutationObject;
use crate::models::profiles::profile_attribute::ProfileAttributeInput;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::PROFILE_ADDED;

pub struct ProfilesMutationObject {}

#[Object(name = "ProfilesMutation")]
impl ProfilesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        profile: ProfileInput,
    ) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.profile.add(ctx, None, &profile, None).await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(PROFILE_ADDED.to_string()),
            profile_id: Some(id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(ctx
            .profile
            .get_by_id(&id)
            .await?
            .map(ProfileObject::new))
    }

    async fn add_collection(
        &self,
        ctx: &Context<'_>,
        profile_id: String,
    ) -> Result<Option<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let id = Uuid::parse_str(&profile_id)?;
        let collection_id = ctx.profile.add_profile_collection(ctx, &id).await?;
        let collection = ctx.content.collections.get(&collection_id).await?;
        Ok(collection.map(CollectionObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        profile: ProfileInput,
    ) -> Result<Option<ProfileObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("not authorized"));
        }
        if let Some(id) = id {
            ctx.check_has_admin_account().await?;
            let id = Uuid::parse_str(&id)?;
            ctx.profile.edit_by_id(ctx, &id, &profile).await?;
            Ok(ctx
                .profile
                .get_by_id(&id)
                .await?
                .map(ProfileObject::new))
        } else {
            let principal_id = ctx.principal.id;
            ctx.profile.edit_by_principal(ctx, &principal_id, &profile).await?;
            Ok(ctx
                .profile
                .get_by_principal(&principal_id)
                .await?
                .map(ProfileObject::new))
        }
    }

    async fn add_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: Vec<ProfileAttributeInput>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_service_account().await?;
        let id = Uuid::parse_str(&id)?;
        ctx.profile.add_attributes(ctx, &id, attributes).await?;
        Ok(true)
    }

    async fn delete_attribute(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = id {
            ctx.check_has_service_account().await?;
            let id = Uuid::parse_str(&id)?;
            let attribute_id = Uuid::parse_str(&attribute_id)?;
            ctx.profile.delete_profile_attribute(&id, &attribute_id).await?;
        } else {
            let Some(profile) = ctx.profile.get_by_principal(&ctx.principal.id).await? else {
                return Err(Error::new("profile not found"));
            };
            let attribute_id = Uuid::parse_str(&attribute_id)?;
            ctx.profile.delete_profile_attribute(&profile.id, &attribute_id).await?;
        }
        Ok(true)
    }

    async fn add_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.add_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn edit_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.edit_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn delete_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.delete_profile_attribute_type(&attribute_id).await?;
        Ok(true)
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<ProfileMutationObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("unauthorized"));
        }
        let Some(profile) = ctx.profile.get_by_principal(&ctx.principal.id).await? else {
            return Err(Error::new("profile not found"));
        };
        Ok(ProfileMutationObject::new(profile))
    }
}
