use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::collection::{Collection, CollectionType};
use async_graphql::{Context, Error, Object, Union};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::category::CategoryObject;
use crate::graphql::content::collection_metadata_relationship::CollectionMetadataRelationshipObject;
use crate::graphql::content::collection_workflow::CollectionWorkflowObject;
use crate::models::content::attributes_filter::AttributesFilterInput;
use crate::models::content::ordering::Ordering;
use crate::models::security::permission::PermissionAction;

#[derive(Union)]
enum CollectionItem {
    Metadata(MetadataObject),
    Collection(CollectionObject),
}

pub struct CollectionObject {
    collection: Collection,
}

impl CollectionObject {
    pub fn new(collection: Collection) -> Self {
        Self { collection }
    }
}

#[Object(name = "Collection")]
impl CollectionObject {
    async fn id(&self) -> String {
        self.collection.id.to_string()
    }

    async fn slug(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.collections.get_slug(&self.collection.id).await
    }

    #[graphql(name = "type")]
    async fn collection_type(&self) -> &CollectionType {
        &self.collection.collection_type
    }

    async fn name(&self) -> &String {
        &self.collection.name
    }

    async fn description(&self) -> &Option<String> {
        &self.collection.description
    }

    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .get_categories(&self.collection.id)
            .await?
            .into_iter()
            .map(CategoryObject::new)
            .collect())
    }

    async fn trait_ids(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.collections.get_trait_ids(&self.collection.id).await
    }

    async fn labels(&self) -> &Vec<String> {
        &self.collection.labels
    }

    async fn attributes(&self, filter: Option<AttributesFilterInput>) -> Option<Value> {
        let mut value = self.collection.attributes.clone();
        if let Some(filter) = filter {
            value = filter.filter(&value);
        }
        if value.is_null() {
            None
        } else {
            Some(value)
        }
    }

    async fn item_attributes(&self) -> &Option<Value> {
        &self.collection.item_attributes
    }

    async fn system_attributes(&self) -> &Option<Value> {
        &self.collection.system_attributes
    }

    async fn metadata_relationships(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CollectionMetadataRelationshipObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .get_metadata_relationships(&self.collection.id)
            .await?
            .into_iter()
            .map(|s| s.into())
            .collect())
    }

    async fn ordering(&self) -> &Option<Vec<Ordering>> {
        &self.collection.ordering
    }

    async fn deleted(&self) -> bool {
        self.collection.deleted
    }

    async fn created(&self) -> &DateTime<Utc> {
        &self.collection.created
    }

    async fn modified(&self) -> &DateTime<Utc> {
        &self.collection.modified
    }

    async fn template_metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        if let Some(id) = &self.collection.template_metadata_id {
            if let Some(version) = &self.collection.template_metadata_version {
                let ctx = ctx.data::<BoscaContext>()?;
                let metadata = ctx.check_metadata_version_action(id, *version, PermissionAction::View).await?;
                return Ok(Some(metadata.into()));
            }
        }
        Ok(None)
    }

    async fn parent_collections(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        Ok(ctx.content
            .collections
            .get_parents(&self.collection.id, offset, limit)
            .await?
            .into_iter()
            .map(|c| c.into())
            .collect())
    }

    async fn items(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionItem>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        let items = ctx.content.collections.get_children(&self.collection, offset, limit).await?;
        let mut content = Vec::new();
        for item in items {
            if let Some(id) = &item.collection_id {
                if let Ok(mut collection) = ctx.check_collection_action(id, PermissionAction::View).await {
                    collection.item_attributes = item.attributes;
                    content.push(CollectionItem::Collection(collection.into()))
                }
            } else if let Some(id) = &item.metadata_id {
                if let Ok(mut metadata) = ctx.check_metadata_action(id, PermissionAction::View).await {
                    metadata.item_attributes = item.attributes;
                    content.push(CollectionItem::Metadata(metadata.into()))
                }
            }
        }
        Ok(content)
    }

    async fn items_count(
        &self,
        ctx: &Context<'_>,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        ctx.content.collections.get_children_count(&self.collection).await
    }

    async fn collections(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        Ok(ctx.content
            .collections
            .get_child_collections(&self.collection, offset, limit)
            .await?
            .into_iter()
            .map(CollectionObject::new)
            .collect())
    }

    async fn collections_count(
        &self,
        ctx: &Context<'_>,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        ctx.content
            .collections
            .get_child_collections_count(&self.collection)
            .await
    }

    async fn metadata(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        Ok(ctx.content
            .collections
            .get_child_metadata(&self.collection, offset, limit)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
    }

    async fn metadata_count(
        &self,
        ctx: &Context<'_>,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        ctx.content
            .collections
            .get_child_metadata_count(&self.collection)
            .await
    }

    async fn workflow(&self) -> CollectionWorkflowObject {
        CollectionWorkflowObject {
            collection: &self.collection,
        }
    }

    async fn ready(&self) -> &Option<DateTime<Utc>> {
        &self.collection.ready
    }

    async fn public(&self) -> bool {
        self.collection.public
    }

    async fn public_list(&self) -> bool {
        self.collection.public_list
    }

    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<PermissionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .collection_permissions
            .get(&self.collection.id)
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}


impl From<Collection> for CollectionObject {
    fn from(collection: Collection) -> Self {
        Self::new(collection)
    }
}
