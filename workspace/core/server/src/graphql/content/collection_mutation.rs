use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_metadata_relationship::CollectionMetadataRelationshipObject;
use crate::graphql::content::collection_supplementary::CollectionSupplementaryObject;
use crate::graphql::content::collection_template_mutation::CollectionTemplateMutationObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::collection::{CollectionChildInput, CollectionInput, CollectionType};
use crate::models::content::collection_metadata_relationship::CollectionMetadataRelationshipInput;
use crate::models::content::collection_supplementary::CollectionSupplementaryInput;
use crate::models::content::collection_workflow_state::{
    CollectionWorkflowCompleteState, CollectionWorkflowState,
};
use crate::models::security::permission::{Permission, PermissionAction, PermissionInput};
use crate::util::delete::delete_collection;
use crate::util::upload::upload_file;
use async_graphql::*;
use bytes::Bytes;
use uuid::Uuid;

pub struct CollectionMutationObject {}

#[Object(name = "CollectionMutation")]
impl CollectionMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        collection: CollectionInput,
        collection_item_attributes: Option<serde_json::Value>,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let is_admin = ctx.has_admin_account().await?;
        if !is_admin && collection.name.to_lowercase().starts_with(".system") {
            return Err(Error::new("invalid collection name"));
        }
        if let Some(collection_type) = collection.collection_type {
            if !is_admin
                && (collection_type == CollectionType::System
                    || collection_type == CollectionType::Root)
            {
                return Err(Error::new("invalid collection type"));
            }
        }
        let mut collections = vec![CollectionChildInput {
            collection,
            attributes: collection_item_attributes,
        }];
        let collection_ids = ctx
            .content
            .collections
            .add_all(ctx, &mut collections)
            .await?;
        let collection = ctx
            .content
            .collections
            .get(collection_ids.first().unwrap())
            .await?;
        Ok(collection.unwrap().into())
    }

    async fn add_bulk(
        &self,
        ctx: &Context<'_>,
        collections: Vec<CollectionChildInput>,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let is_admin = ctx.has_admin_account().await?;
        for collection in collections.iter() {
            if !is_admin
                && collection
                    .collection
                    .name
                    .to_lowercase()
                    .starts_with(".system")
            {
                return Err(Error::new("invalid collection name"));
            }
            if let Some(collection_type) = collection.collection.collection_type {
                if !is_admin
                    && (collection_type == CollectionType::System
                        || collection_type == CollectionType::Root)
                {
                    return Err(Error::new("invalid collection type"));
                }
            }
        }
        let mut collections = collections;
        let collection_ids = ctx
            .content
            .collections
            .add_all(ctx, &mut collections)
            .await?;
        let mut collections = Vec::new();
        for id in collection_ids {
            let collection = ctx.content.collections.get(&id).await?.unwrap();
            collections.push(collection.into());
        }
        Ok(collections)
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        collection: CollectionInput,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let is_admin = ctx.has_admin_account().await?;
        if !is_admin && collection.name.to_lowercase().starts_with(".system") {
            return Err(Error::new("invalid collection name"));
        }
        if let Some(collection_type) = collection.collection_type {
            if !is_admin
                && (collection_type == CollectionType::System
                    || collection_type == CollectionType::Root)
            {
                return Err(Error::new("invalid collection type"));
            }
        }
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .edit(ctx, &collection_id, &collection)
            .await?;
        match ctx.content.collections.get(&collection_id).await? {
            Some(collection) => Ok(collection.into()),
            None => Err(Error::new("Error creating collection")),
        }
    }

    async fn set_template(
        &self,
        ctx: &Context<'_>,
        collection_id: String,
        template_id: String,
        template_version: i32,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(collection_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Manage);
        ctx.collection_permission_check(check).await?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        ctx.content
            .collections
            .set_template(ctx, &id, &template_id, template_version)
            .await?;
        Ok(true)
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
        recursive: Option<bool>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        delete_collection(ctx, &collection_id, recursive, false).await?;
        Ok(true)
    }

    async fn permanently_delete(
        &self,
        ctx: &Context<'_>,
        collection_id: String,
        recursive: Option<bool>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(collection_id.as_str())?;
        ctx.check_has_admin_account().await?;
        delete_collection(ctx, &id, recursive, true).await?;
        Ok(true)
    }

    async fn add_metadata_relationship(
        &self,
        ctx: &Context<'_>,
        relationship: CollectionMetadataRelationshipInput,
    ) -> Result<CollectionMetadataRelationshipObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(relationship.id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        let check = PermissionCheck::new_with_metadata_id(metadata_id, PermissionAction::Edit);
        let metadata = ctx.metadata_permission_check(check).await?;
        ctx.content
            .collections
            .add_metadata_relationship(ctx, &relationship)
            .await?;
        match ctx
            .content
            .collections
            .get_metadata_relationship(&id, &metadata_id)
            .await?
        {
            Some(relationship) => Ok(CollectionMetadataRelationshipObject::new(relationship, metadata)),
            None => Err(Error::new("error creating relationship")),
        }
    }

    async fn edit_metadata_relationship(
        &self,
        ctx: &Context<'_>,
        relationship: CollectionMetadataRelationshipInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(relationship.id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        let check = PermissionCheck::new_with_metadata_id(metadata_id, PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        ctx.content
            .collections
            .edit_metadata_relationship(ctx, &relationship)
            .await?;
        ctx.content
            .collections
            .get_metadata_relationship(&id, &metadata_id)
            .await?;
        Ok(true)
    }

    async fn delete_metadata_relationship(
        &self,
        ctx: &Context<'_>,
        id: String,
        metadata_id: String,
        relationship: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let metadata_id = Uuid::parse_str(metadata_id.as_str())?;
        let check = PermissionCheck::new_with_metadata_id(metadata_id, PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        ctx.content
            .collections
            .delete_metadata_relationship(ctx, &id, &metadata_id, &relationship)
            .await?;
        Ok(true)
    }

    async fn set_public(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let mut collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content.collections.set_public(ctx, &id, public).await?;
        collection.public = public;
        Ok(collection.into())
    }

    async fn set_public_list(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let mut collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .set_public_list(ctx, &id, public)
            .await?;
        collection.public_list = public;
        Ok(collection.into())
    }

    async fn set_public_supplementary(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Manage);
        let mut collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collection_supplementary
            .set_supplementary_public(ctx, &id, public)
            .await?;
        collection.public = public;
        Ok(collection.into())
    }

    async fn set_locked(
        &self,
        ctx: &Context<'_>,
        id: String,
        locked: bool,
    ) -> Result<Option<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        ctx.collection_permission_check(check).await?;
        ctx.content.collections.set_locked(ctx, &id, locked).await?;
        Ok(ctx
            .content
            .collections
            .get(&id)
            .await?
            .map(|collection| collection.into()))
    }

    async fn add_permission(
        &self,
        ctx: &Context<'_>,
        permission: PermissionInput,
    ) -> Result<PermissionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission: Permission = permission.into();
        let check =
            PermissionCheck::new_with_collection_id(permission.entity_id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collection_permissions
            .add(ctx, &permission)
            .await?;
        Ok(permission.into())
    }

    async fn delete_permission(
        &self,
        ctx: &Context<'_>,
        permission: PermissionInput,
    ) -> Result<PermissionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission: Permission = permission.into();
        let check =
            PermissionCheck::new_with_collection_id(permission.entity_id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collection_permissions
            .delete(ctx, &permission)
            .await?;
        Ok(permission.into())
    }

    async fn set_child_item_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        child_collection_id: Option<String>,
        child_metadata_id: Option<String>,
        attributes: Option<serde_json::Value>,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Manage);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let child_collection_id = child_collection_id.map(|c| Uuid::parse_str(c.as_str()).unwrap());
        let child_metadata_id = child_metadata_id.map(|c| Uuid::parse_str(c.as_str()).unwrap());
        ctx.content
            .collections
            .set_child_item_attributes(ctx, &id, child_collection_id, child_metadata_id, attributes)
            .await?;
        Ok(collection.into())
    }

    async fn add_child_collection(
        &self,
        ctx: &Context<'_>,
        id: String,
        collection_id: String,
        attributes: Option<serde_json::Value>,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let collection_id = Uuid::parse_str(collection_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .add_child_collection(ctx, &id, &collection_id, &attributes)
            .await?;
        Ok(collection.into())
    }

    async fn remove_child_collection(
        &self,
        ctx: &Context<'_>,
        id: String,
        collection_id: String,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let collection_id = Uuid::parse_str(collection_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .remove_child_collection(ctx, &id, &collection_id)
            .await?;
        Ok(collection.into())
    }

    async fn add_child_metadata(
        &self,
        ctx: &Context<'_>,
        id: String,
        metadata_id: String,
        attributes: Option<serde_json::Value>,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let metadata_id = Uuid::parse_str(metadata_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .add_child_metadata(ctx, &id, &metadata_id, &attributes)
            .await?;
        Ok(collection.into())
    }

    async fn remove_child_metadata(
        &self,
        ctx: &Context<'_>,
        id: String,
        metadata_id: String,
    ) -> Result<CollectionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let metadata_id = Uuid::parse_str(metadata_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .remove_child_metadata(ctx, &id, &metadata_id)
            .await?;
        Ok(collection.into())
    }

    async fn set_workflow_state(
        &self,
        ctx: &Context<'_>,
        state: CollectionWorkflowState,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(state.collection_id.as_str())?;
        ctx.check_has_service_account().await?;
        if let Some(collection) = ctx.content.collections.get(&id).await? {
            ctx.content
                .collection_workflows
                .set_state(
                    ctx,
                    &ctx.principal,
                    &collection,
                    &state.state_id,
                    None,
                    &state.status,
                    true,
                    state.immediate,
                )
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn set_workflow_state_complete(
        &self,
        ctx: &Context<'_>,
        state: CollectionWorkflowCompleteState,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(state.collection_id.as_str())?;
        ctx.check_has_service_account().await?;
        if let Some(collection) = ctx.content.collections.get(&id).await? {
            let mut state_id = collection.workflow_state_id.clone();
            if collection.workflow_state_pending_id.is_some() {
                state_id = collection.workflow_state_pending_id.clone().unwrap();
            }
            ctx.content
                .collection_workflows
                .set_state(
                    ctx,
                    &ctx.principal,
                    &collection,
                    &state_id,
                    None,
                    &state.status,
                    true,
                    true,
                )
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn set_collection_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check =
            PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .set_attributes(ctx, &collection_id, attributes)
            .await?;
        Ok(true)
    }

    async fn set_collection_system_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check =
            PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .set_system_attributes(ctx, &collection_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_collection_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .merge_attributes(ctx, &collection_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_collection_item_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        item_id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let collection_item_id = Uuid::parse_str(item_id.as_str())?;
        ctx.content
            .collections
            .merge_collection_item_attributes(ctx, &collection_id, &collection_item_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_metadata_item_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        item_id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.items_locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let metadata_item_id = Uuid::parse_str(item_id.as_str())?;
        ctx.content
            .collections
            .merge_metadata_item_attributes(ctx, &collection_id, &metadata_item_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_metadata_relationship_attributes(
        &self,
        ctx: &Context<'_>,
        collection_id: String,
        metadata_id: String,
        relationship: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(collection_id.as_str())?;
        let metadata_id = Uuid::parse_str(metadata_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let check = PermissionCheck::new_with_metadata_id(metadata_id, PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        ctx.content
            .collections
            .merge_relationship_attributes(
                ctx,
                &collection_id,
                &metadata_id,
                &relationship,
                attributes,
            )
            .await?;
        Ok(true)
    }

    async fn set_collection_ordering(
        &self,
        ctx: &Context<'_>,
        id: String,
        ordering: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check =
            PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collections
            .set_ordering(ctx, &collection_id, ordering)
            .await?;
        Ok(true)
    }

    async fn set_categories(
        &self,
        ctx: &Context<'_>,
        id: String,
        category_ids: Vec<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Edit);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let mut ids = Vec::new();
        for category_id in category_ids {
            let id = Uuid::parse_str(&category_id)?;
            ids.push(id);
        }
        ctx.content
            .collections
            .set_categories(ctx, &id, &ids)
            .await?;
        Ok(true)
    }

    async fn set_ready(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Edit);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.ready.is_some() {
            return Err(Error::new("collection already ready"));
        }
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collection_workflows
            .set_ready_and_enqueue(ctx, &ctx.principal, &collection, None)
            .await?;
        Ok(true)
    }

    async fn add_supplementary(
        &self,
        ctx: &Context<'_>,
        supplementary: CollectionSupplementaryInput,
    ) -> Result<CollectionSupplementaryObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(&supplementary.collection_id)?;
        let check =
            PermissionCheck::new_with_collection_id(collection_id, PermissionAction::Manage);
        let collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let id = ctx
            .content
            .collection_supplementary
            .add_supplementary(ctx, &supplementary)
            .await?;
        match ctx
            .content
            .collection_supplementary
            .get_supplementary(&id)
            .await?
        {
            Some(supplementary) => Ok(CollectionSupplementaryObject::new(
                collection,
                supplementary,
            )),
            None => Err(Error::new("Error creating metadata")),
        }
    }

    async fn set_supplementary_uploaded(
        &self,
        ctx: &Context<'_>,
        supplementary_id: String,
        content_type: String,
        len: usize,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(supplementary_id.as_str())?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::Manage);
        let c = ctx.collection_permission_check(check).await?;
        if c.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        ctx.content
            .collection_supplementary
            .set_supplementary_uploaded(ctx, &id, content_type.as_str(), len)
            .await?;
        Ok(true)
    }

    async fn set_supplementary_contents(
        &self,
        ctx: &Context<'_>,
        supplementary_id: String,
        content_type: String,
        file: Upload,
    ) -> Result<bool, Error> {
        let octx = ctx;
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::parse_str(supplementary_id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .collection_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let check = PermissionCheck::new_with_collection_id(
            supplementary.collection_id,
            PermissionAction::Manage,
        );
        let collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let path = ctx
            .storage
            .get_collection_path(&collection, Some(supplementary.id))
            .await?;
        let len = upload_file(ctx, octx, path, file).await?;
        ctx.content
            .collection_supplementary
            .set_supplementary_uploaded(ctx, &supplementary_id, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_supplementary_text_contents(
        &self,
        ctx: &Context<'_>,
        supplementary_id: String,
        content_type: String,
        content: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::parse_str(supplementary_id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .collection_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let check = PermissionCheck::new_with_collection_id(
            supplementary.collection_id,
            PermissionAction::Manage,
        );
        let collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let path = ctx
            .storage
            .get_collection_path(&collection, Some(supplementary_id))
            .await?;
        let bytes: Bytes = content.into();
        let len = bytes.len();
        ctx.storage.put(&path, bytes).await?;
        ctx.content
            .collection_supplementary
            .set_supplementary_uploaded(ctx, &supplementary_id, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn delete_supplementary(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::parse_str(id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .collection_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let check = PermissionCheck::new_with_collection_id(
            supplementary.collection_id,
            PermissionAction::Manage,
        );
        let collection = ctx.collection_permission_check(check).await?;
        if collection.locked && !ctx.has_service_account().await? {
            return Err(Error::new("locked"));
        }
        let path = ctx
            .storage
            .get_collection_path(&collection, Some(supplementary_id))
            .await?;
        ctx.storage.delete(&path).await?;
        ctx.content
            .collection_supplementary
            .delete_supplementary(ctx, &supplementary_id)
            .await?;
        Ok(true)
    }

    async fn template(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
    ) -> Result<CollectionTemplateMutationObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&metadata_id)?;
        let check = PermissionCheck::new_with_metadata_id_with_version(
            id,
            metadata_version,
            PermissionAction::Edit,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(CollectionTemplateMutationObject::new(metadata))
    }
}
