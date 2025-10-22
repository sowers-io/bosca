use crate::caching_headers::CachingHeaderManager;
use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::category::CategoryObject;
use crate::graphql::content::collection_metadata_relationship::CollectionMetadataRelationshipObject;
use crate::graphql::content::collection_supplementary::CollectionSupplementaryObject;
use crate::graphql::content::collection_workflow::CollectionWorkflowObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::attributes_filter::AttributesFilterInput;
use crate::models::content::collection::{Collection, CollectionType};
use crate::models::content::ordering::Ordering;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object, Union};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

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

    async fn etag(&self, ctx: &Context<'_>, add_header: bool) -> Result<&Option<String>, Error> {
        if add_header {
            let caching = CachingHeaderManager::get(ctx)?;
            caching.apply(ctx, &self.collection);
        }
        Ok(&self.collection.etag)
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

    async fn locked(&self) -> bool {
        self.collection.locked
    }

    async fn items_locked(&self) -> bool {
        self.collection.items_locked
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
        ctx.content
            .collections
            .get_trait_ids(&self.collection.id)
            .await
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
        let relationships = ctx
            .content
            .collections
            .get_metadata_relationships(&self.collection.id)
            .await?;
        let mut rels = Vec::new();
        for r in relationships {
            let check = PermissionCheck::new_with_metadata_id_advertised(
                r.metadata_id,
                PermissionAction::View,
            );
            if let Ok(metadata) = ctx.metadata_permission_check(check).await {
                rels.push(CollectionMetadataRelationshipObject::new(r, metadata));
            }
        }
        Ok(rels)
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
                let check = PermissionCheck::new_with_metadata_id_with_version(
                    *id,
                    *version,
                    PermissionAction::View,
                );
                let metadata = ctx.metadata_permission_check(check).await?;
                return Ok(Some(metadata.into()));
            }
        }
        Ok(None)
    }

    async fn parent_collections(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_collection_id(
            self.collection.id,
            PermissionAction::List,
        );
        ctx.collection_permission_check(check).await?;
        Ok(ctx
            .content
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
        state: Option<String>,
        language_tag: Option<String>,
    ) -> Result<Vec<CollectionItem>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_collection_id(
            self.collection.id,
            PermissionAction::List,
        );
        ctx.collection_permission_check(check).await?;
        let items = ctx
            .content
            .collections
            .get_children(&self.collection, offset, limit, &state, &language_tag)
            .await?;
        let mut content = Vec::new();
        for item in items {
            if let Some(id) = &item.collection_id {
                let check =
                    PermissionCheck::new_with_collection_id(*id, PermissionAction::View);
                if let Ok(mut collection) = ctx.collection_permission_check(check).await {
                    collection.item_attributes = item.attributes;
                    content.push(CollectionItem::Collection(collection.into()))
                }
            } else if let Some(id) = &item.metadata_id {
                let check = PermissionCheck::new_with_metadata_id_advertised(
                    *id,
                    PermissionAction::View,
                );
                if let Ok(mut metadata) = ctx.metadata_permission_check(check).await {
                    metadata.item_attributes = item.attributes;
                    content.push(CollectionItem::Metadata(metadata.into()))
                }
            }
        }
        Ok(content)
    }

    async fn items_count(&self, ctx: &Context<'_>, state: Option<String>, language_tag: Option<String>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        ctx.content
            .collections
            .get_children_count(&self.collection, &state, &language_tag)
            .await
    }

    async fn collections(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        let mut result = Vec::new();
        let collections = ctx
            .content
            .collections
            .get_child_collections(&self.collection, offset, limit)
            .await?;
        for c in collections {
            let check = PermissionCheck::new_with_collection(c, PermissionAction::View);
            if let Ok(collection) = ctx.collection_permission_check(check).await {
                result.push(collection);
            }
        }
        Ok(result.into_iter().map(CollectionObject::new).collect())
    }

    async fn collections_count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
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
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        let metadata = ctx
            .content
            .collections
            .get_child_metadata(&self.collection, offset, limit)
            .await?;
        let mut result = Vec::new();
        for m in metadata {
            let check = PermissionCheck::new_with_metadata_advertised(m, PermissionAction::View);
            if let Ok(metadata) = ctx.metadata_permission_check(check).await {
                result.push(metadata);
            }
        }
        Ok(result.into_iter().map(MetadataObject::new).collect())
    }

    async fn metadata_count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        ctx.content
            .collections
            .get_child_metadata_count(&self.collection)
            .await
    }

    async fn expanded_metadata(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
        state: Option<String>,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        let items = ctx
            .content
            .collections
            .get_expanded_metadata(&self.collection, offset, limit, &state)
            .await?;
        let mut content = Vec::new();
        for item in items {
            if let Some(id) = &item.metadata_id {
                let check = PermissionCheck::new_with_metadata_id_advertised(
                    *id,
                    PermissionAction::View,
                );
                if let Ok(mut metadata) = ctx.metadata_permission_check(check).await {
                    metadata.item_attributes = item.attributes;
                    content.push(metadata.into())
                }
            }
        }
        Ok(content)
    }

    async fn expanded_metadata_count(
        &self,
        ctx: &Context<'_>,
        state: Option<String>,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection_id(self.collection.id, PermissionAction::List);
        ctx.collection_permission_check(check).await?;
        ctx.content
            .collections
            .get_expanded_metadata_count(&self.collection, &state)
            .await
    }

    async fn workflow(&self) -> CollectionWorkflowObject<'_> {
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

    async fn public_supplementary(&self) -> bool {
        self.collection.public_supplementary
    }

    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<PermissionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_collection(self.collection.clone(), PermissionAction::Manage);
        ctx.metadata_permission_check(check).await?;
        Ok(ctx
            .content
            .collection_permissions
            .get(&self.collection.id)
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }

    async fn supplementary(
        &self,
        ctx: &Context<'_>,
        key: Option<String>,
        plan_id: Option<String>,
    ) -> Result<Vec<CollectionSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(key) = key {
            let check = PermissionCheck::new_with_collection_id_supplementary(
                self.collection.id,
                PermissionAction::View,
            );
            if ctx.collection_permission_check(check).await.is_err() {
                return Ok(vec![]);
            }
            let plan_id = plan_id.map(|p| Uuid::parse_str(&p).unwrap());
            if let Some(supplementary) = ctx
                .content
                .collection_supplementary
                .get_supplementary_by_key(&self.collection.id, &key, plan_id)
                .await?
            {
                return Ok(vec![CollectionSupplementaryObject::new(
                    self.collection.clone(),
                    supplementary,
                )]);
            }
            return Ok(vec![]);
        }
        let check = PermissionCheck::new_with_collection_id_supplementary(
            self.collection.id,
            PermissionAction::List,
        );
        if ctx.collection_permission_check(check).await.is_err() {
            return Ok(vec![]);
        }
        Ok(ctx
            .content
            .collection_supplementary
            .get_supplementaries(&self.collection.id)
            .await?
            .into_iter()
            .map(|s| CollectionSupplementaryObject::new(self.collection.clone(), s))
            .collect())
    }
}

impl From<Collection> for CollectionObject {
    fn from(collection: Collection) -> Self {
        Self::new(collection)
    }
}
