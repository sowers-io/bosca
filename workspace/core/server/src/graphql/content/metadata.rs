use crate::caching_headers::CachingHeaderManager;
use crate::context::BoscaContext;
use crate::graphql::content::bible::BibleObject;
use crate::graphql::content::category::CategoryObject;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_template::CollectionTemplateObject;
use crate::graphql::content::document::DocumentObject;
use crate::graphql::content::document_template::DocumentTemplateObject;
use crate::graphql::content::guide::GuideObject;
use crate::graphql::content::guide_template::GuideTemplateObject;
use crate::graphql::content::metadata_content::MetadataContentObject;
use crate::graphql::content::metadata_profile::MetadataProfileObject;
use crate::graphql::content::metadata_relationship::MetadataRelationshipObject;
use crate::graphql::content::metadata_source::MetadataSourceObject;
use crate::graphql::content::metadata_supplementary::MetadataSupplementaryObject;
use crate::graphql::content::metadata_workflow::MetadataWorkflowObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::attributes_filter::AttributesFilterInput;
use crate::models::content::metadata::{Metadata, MetadataType};
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use crate::graphql::content::document_collaboration::DocumentCollaborationObject;

pub struct MetadataObject {
    metadata: Metadata,
}

impl MetadataObject {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

#[Object(name = "Metadata")]
impl MetadataObject {
    async fn id(&self) -> String {
        self.metadata.id.to_string()
    }

    async fn etag(&self, ctx: &Context<'_>, add_header: bool) -> Result<&Option<String>, Error> {
        if add_header {
            let caching = CachingHeaderManager::get(ctx)?;
            caching.apply(ctx, &self.metadata);
        }
        Ok(&self.metadata.etag)
    }

    async fn parent_id(&self) -> Option<String> {
        self.metadata.parent_id.map(|id| id.to_string())
    }

    async fn version(&self) -> i32 {
        self.metadata.version
    }

    async fn locked(&self) -> bool {
        self.metadata.locked
    }

    async fn trait_ids(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = BoscaContext::get(ctx)?;
        ctx.content.metadata.get_trait_ids(&self.metadata.id).await
    }

    async fn slug(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.metadata.get_slug(&self.metadata.id).await
    }

    #[graphql(name = "type")]
    async fn metadata_type(&self) -> &MetadataType {
        &self.metadata.metadata_type
    }

    async fn name(&self) -> &String {
        &self.metadata.name
    }

    async fn content(&self) -> MetadataContentObject {
        MetadataContentObject {
            metadata: self.metadata.clone(),
        }
    }

    async fn language_tag(&self) -> &String {
        &self.metadata.language_tag
    }

    async fn labels(&self) -> &Vec<String> {
        &self.metadata.labels
    }

    async fn attributes(&self, filter: Option<AttributesFilterInput>) -> Option<Value> {
        let mut value = self.metadata.attributes.clone();
        if let Some(filter) = filter {
            value = filter.filter(&value);
        }
        if value.is_null() {
            None
        } else {
            Some(value)
        }
    }

    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .get_categories(&self.metadata.id)
            .await?
            .into_iter()
            .map(CategoryObject::new)
            .collect())
    }

    async fn item_attributes(&self) -> &Option<Value> {
        &self.metadata.item_attributes
    }

    async fn system_attributes(&self, ctx: &Context<'_>) -> Result<&Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        match ctx
            .check_metadata_action(&self.metadata.id, PermissionAction::Manage)
            .await
        {
            Ok(_) => Ok(&self.metadata.system_attributes),
            Err(_) => Ok(&None),
        }
    }

    async fn created(&self) -> &DateTime<Utc> {
        &self.metadata.created
    }

    async fn modified(&self) -> &DateTime<Utc> {
        &self.metadata.modified
    }

    async fn uploaded(&self) -> &Option<DateTime<Utc>> {
        &self.metadata.uploaded
    }

    async fn ready(&self) -> &Option<DateTime<Utc>> {
        &self.metadata.ready
    }

    async fn deleted(&self) -> bool {
        self.metadata.deleted
    }

    async fn workflow(&self) -> MetadataWorkflowObject {
        MetadataWorkflowObject {
            metadata: self.metadata.clone(),
        }
    }

    async fn source(&self) -> MetadataSourceObject {
        MetadataSourceObject {
            metadata: self.metadata.clone(),
        }
    }

    async fn public(&self) -> bool {
        self.metadata.public
    }

    async fn public_content(&self) -> bool {
        self.metadata.public_content
    }

    async fn public_supplementary(&self) -> bool {
        self.metadata.public_supplementary
    }

    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<PermissionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata_permissions
            .get_metadata_permissions(&self.metadata.id)
            .await?
            .into_iter()
            .map(Permission::into)
            .collect())
    }

    async fn bible(&self, ctx: &Context<'_>, variant: Option<String>) -> Result<Option<BibleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let bible = ctx
            .content
            .bibles
            .get_bible(&self.metadata.id, self.metadata.version, variant)
            .await?;
        Ok(bible.map(BibleObject::new))
    }

    async fn document(&self, ctx: &Context<'_>) -> Result<Option<DocumentObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let document = ctx
            .content
            .documents
            .get_document(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(DocumentObject::new))
    }

    async fn document_collaboration(&self, ctx: &Context<'_>) -> Result<Option<DocumentCollaborationObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let document = ctx
            .content
            .documents
            .get_document_collaboration(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(DocumentCollaborationObject::new))
    }

    async fn document_template(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<DocumentTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let document = ctx
            .content
            .documents
            .get_template(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(DocumentTemplateObject::new))
    }

    async fn guide(&self, ctx: &Context<'_>) -> Result<Option<GuideObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let guide = ctx
            .content
            .guides
            .get_guide(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(guide.map(GuideObject::new))
    }

    async fn guide_template(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<GuideTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let guide = ctx
            .content
            .guides
            .get_template(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(guide.map(GuideTemplateObject::new))
    }

    async fn collection_template(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<CollectionTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let document = ctx
            .content
            .collection_templates
            .get_template(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(CollectionTemplateObject::new))
    }

    async fn profiles(&self, ctx: &Context<'_>) -> Result<Vec<MetadataProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .get_profiles(&self.metadata.id)
            .await?
            .into_iter()
            .map(MetadataProfileObject::new)
            .collect())
    }

    async fn relationships(
        &self,
        ctx: &Context<'_>,
        filter: Option<Vec<String>>,
        inverse: Option<bool>,
    ) -> Result<Vec<MetadataRelationshipObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let relationships = if inverse.unwrap_or(false) {
            ctx.content
                .metadata
                .get_relationships_inverse(&self.metadata.id)
                .await?
        } else {
            ctx.content
                .metadata
                .get_relationships(&self.metadata.id)
                .await?
        };
        Ok(relationships
            .into_iter()
            .filter(|r| {
                if let Some(filter) = &filter {
                    filter.contains(&r.relationship)
                } else {
                    true
                }
            })
            .map(|s| s.into())
            .collect())
    }

    async fn supplementary(
        &self,
        ctx: &Context<'_>,
        key: Option<String>,
        plan_id: Option<String>,
    ) -> Result<Vec<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;

        if let Some(key) = key {
            if ctx
                .check_metadata_supplementary_action(&self.metadata, PermissionAction::View)
                .await
                .is_err()
            {
                return Ok(vec![]);
            }
            if let Some(plan_id) = plan_id.map(|p| Uuid::parse_str(&p).unwrap()) {
                if let Some(supplementary) = ctx
                    .content
                    .metadata_supplementary
                    .get_supplementary_by_key_and_plan_id(&self.metadata.id, &key, &plan_id)
                    .await?
                {
                    return Ok(vec![MetadataSupplementaryObject::new(
                        self.metadata.clone(),
                        supplementary,
                    )]);
                }
            } else if let Some(supplementary) = ctx
                .content
                .metadata_supplementary
                .get_supplementary_by_key(&self.metadata.id, &key)
                .await?
            {
                return Ok(vec![MetadataSupplementaryObject::new(
                    self.metadata.clone(),
                    supplementary,
                )]);
            }

            return Ok(vec![]);
        }

        if ctx
            .check_metadata_supplementary_action(&self.metadata, PermissionAction::List)
            .await
            .is_err()
        {
            return Ok(vec![]);
        }

        Ok(ctx
            .content
            .metadata_supplementary
            .get_supplementaries(&self.metadata.id)
            .await?
            .into_iter()
            .map(|s| MetadataSupplementaryObject::new(self.metadata.clone(), s))
            .collect())
    }

    async fn parent_collections(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collections = ctx
            .content
            .metadata
            .get_parent_ids(&self.metadata.id, offset, limit)
            .await?;
        let mut listable = Vec::new();
        for id in collections {
            if let Ok(collection) = ctx
                .check_collection_action(&id, PermissionAction::List)
                .await
            {
                listable.push(collection.into());
            }
        }
        Ok(listable)
    }
}

impl From<Metadata> for MetadataObject {
    fn from(metadata: Metadata) -> Self {
        Self::new(metadata)
    }
}
