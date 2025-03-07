use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_metadata_relationship::CollectionMetadataRelationshipObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::collection::{CollectionChildInput, CollectionInput, CollectionType};
use crate::models::content::collection_metadata_relationship::CollectionMetadataRelationshipInput;
use crate::models::content::collection_workflow_state::{
    CollectionWorkflowCompleteState, CollectionWorkflowState,
};
use crate::models::content::search::SearchDocumentInput;
use crate::models::security::permission::{Permission, PermissionAction, PermissionInput};
use crate::util::delete::delete_collection;
use crate::util::storage::index_documents;
use async_graphql::*;
use log::error;
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
            if !is_admin && (collection_type == CollectionType::System || collection_type == CollectionType::Root)
            {
                return Err(Error::new("invalid collection type"));
            }
        }
        let collection_id = Uuid::parse_str(id.as_str())?;
        ctx.check_collection_action(&collection_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .edit(&collection_id, &collection)
            .await?;
        if collection.index.unwrap_or(true) {
            let storage_system = ctx.workflow.get_default_search_storage_system().await?;
            let documents = vec![SearchDocumentInput {
                metadata_id: None,
                collection_id: Some(collection_id.to_string()),
                profile_id: None,
                content: "".to_owned(),
            }];
            if let Some(storage_system) = storage_system {
                index_documents(ctx, &documents, &storage_system).await?;
            } else {
                error!("failed to index, default search storage system not configured");
            }
        }
        match ctx.content.collections.get(&collection_id).await? {
            Some(collection) => Ok(collection.into()),
            None => Err(Error::new("Error creating collection")),
        }
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
        ctx.check_collection_action(&id, PermissionAction::Edit)
            .await?;
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .add_metadata_relationship(&relationship)
            .await?;
        match ctx
            .content
            .collections
            .get_metadata_relationship(&id, &metadata_id)
            .await?
        {
            Some(relationship) => Ok(relationship.into()),
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
        ctx.check_collection_action(&id, PermissionAction::Edit)
            .await?;
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .edit_metadata_relationship(&relationship)
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
        ctx.check_collection_action(&id, PermissionAction::Edit)
            .await?;
        let metadata_id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .delete_metadata_relationship(&id, &metadata_id, &relationship)
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
        let mut collection = ctx
            .check_collection_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.collections.set_public(&id, public).await?;
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
        let mut collection = ctx
            .check_collection_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.collections.set_public_list(&id, public).await?;
        collection.public_list = public;
        Ok(collection.into())
    }

    async fn add_permission(
        &self,
        ctx: &Context<'_>,
        permission: PermissionInput,
    ) -> Result<PermissionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission: Permission = permission.into();
        ctx.check_collection_action(&permission.entity_id, PermissionAction::Manage)
            .await?;
        ctx.content.collection_permissions.add(&permission).await?;
        Ok(permission.into())
    }

    async fn delete_permission(
        &self,
        ctx: &Context<'_>,
        permission: PermissionInput,
    ) -> Result<PermissionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission: Permission = permission.into();
        ctx.check_collection_action(&permission.entity_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .collection_permissions
            .delete(&permission)
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
        let collection = ctx
            .check_collection_action(&id, PermissionAction::Manage)
            .await?;
        let child_collection_id = child_collection_id.map(|c| Uuid::parse_str(c.as_str()).unwrap());
        let child_metadata_id = child_metadata_id.map(|c| Uuid::parse_str(c.as_str()).unwrap());
        ctx.content
            .collections
            .set_child_item_attributes(&id, child_collection_id, child_metadata_id, attributes)
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
        let collection = ctx
            .check_collection_action(&collection_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .add_child_collection(&id, &collection_id, &attributes)
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
        let collection = ctx
            .check_collection_action(&collection_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .remove_child_collection(&id, &collection_id)
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
        let collection = ctx
            .check_collection_action(&id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .add_child_metadata(&id, &metadata_id, &attributes)
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
        let collection = ctx
            .check_collection_action(&id, PermissionAction::Edit)
            .await?;
        ctx.content
            .collections
            .remove_child_metadata(&id, &metadata_id)
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
        ctx.check_collection_action(&collection_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .collections
            .set_attributes(&collection_id, attributes)
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
        ctx.check_collection_action(&collection_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .collections
            .set_ordering(&collection_id, ordering)
            .await?;
        Ok(true)
    }

    async fn set_ready(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collection_id = Uuid::parse_str(id.as_str())?;
        let collection = ctx
            .check_collection_action(&collection_id, PermissionAction::Manage)
            .await?;
        if collection.ready.is_some() {
            return Err(Error::new("collection already ready"));
        }
        ctx.content
            .collection_workflows
            .set_ready_and_enqueue(&ctx.workflow, &ctx.principal, &collection, None)
            .await?;
        Ok(true)
    }
}
