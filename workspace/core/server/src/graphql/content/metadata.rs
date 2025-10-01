use crate::caching_headers::CachingHeaderManager;
use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::bible::BibleObject;
use crate::graphql::content::category::CategoryObject;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_template::CollectionTemplateObject;
use crate::graphql::content::comment::CommentsObject;
use crate::graphql::content::document::DocumentObject;
use crate::graphql::content::document_collaboration::DocumentCollaborationObject;
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
use crate::models::workflow::states::ADVERTISED;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashSet;
use std::string::ToString;
use uuid::Uuid;

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
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            ctx.content.metadata.get_trait_ids(&self.metadata.id).await
        } else {
            Ok(vec![])
        }
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

    async fn labels(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(self.metadata.labels.clone())
        } else {
            Ok(vec![])
        }
    }

    async fn attributes(
        &self,
        ctx: &Context<'_>,
        filter: Option<AttributesFilterInput>,
    ) -> Result<Option<Value>, Error> {
        if self.metadata.attributes.is_null() {
            return Ok(None);
        }
        let value = self.metadata.attributes.clone();
        if self.metadata.workflow_state_id == ADVERTISED {
            let ctx = ctx.data::<BoscaContext>()?;
            let check =
                PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
            if !ctx.metadata_permission_check(check).await.is_ok() {
                let mut attrs = HashSet::new();
                attrs.insert("type".to_string());
                attrs.insert("description".to_string());
                attrs.insert("published".to_string());
                let filter = AttributesFilterInput {
                    attributes: attrs,
                    child_attributes: None,
                };
                return Ok(Some(filter.filter(&value)));
            }
        }
        if let Some(filter) = filter {
            return Ok(Some(filter.filter(&value)));
        }
        Ok(Some(value))
    }

    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(ctx
                .content
                .metadata
                .get_categories(&self.metadata.id)
                .await?
                .into_iter()
                .map(CategoryObject::new)
                .collect())
        } else {
            Ok(vec![])
        }
    }

    async fn item_attributes(&self, ctx: &Context<'_>) -> Result<&Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(&self.metadata.item_attributes)
        } else {
            Ok(&None)
        }
    }

    async fn system_attributes(&self, ctx: &Context<'_>) -> Result<&Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Manage);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(&self.metadata.system_attributes)
        } else {
            Ok(&None)
        }
    }

    async fn created(&self) -> &DateTime<Utc> {
        &self.metadata.created
    }

    async fn modified(&self, ctx: &Context<'_>) -> Result<Option<&DateTime<Utc>>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(Some(&self.metadata.modified))
        } else {
            Ok(None)
        }
    }

    async fn uploaded(&self, ctx: &Context<'_>) -> Result<&Option<DateTime<Utc>>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(&self.metadata.uploaded)
        } else {
            Ok(&None)
        }
    }

    async fn ready(&self, ctx: &Context<'_>) -> Result<&Option<DateTime<Utc>>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(&self.metadata.ready)
        } else {
            Ok(&None)
        }
    }

    async fn deleted(&self) -> bool {
        self.metadata.deleted
    }

    async fn workflow(&self, ctx: &Context<'_>) -> Result<Option<MetadataWorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(Some(MetadataWorkflowObject {
                metadata: self.metadata.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn source(&self, ctx: &Context<'_>) -> Result<Option<MetadataSourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(Some(MetadataSourceObject {
                metadata: self.metadata.clone(),
            }))
        } else {
            Ok(None)
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
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Manage);
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(ctx
                .content
                .metadata_permissions
                .get_metadata_permissions(&self.metadata.id)
                .await?
                .into_iter()
                .map(Permission::into)
                .collect())
        } else {
            Ok(vec![])
        }
    }

    async fn bible(
        &self,
        ctx: &Context<'_>,
        variant: Option<String>,
    ) -> Result<Option<BibleObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            let bible = ctx
                .content
                .bibles
                .get_bible(&self.metadata.id, self.metadata.version, variant)
                .await?;
            Ok(bible.map(BibleObject::new))
        } else {
            Ok(None)
        }
    }

    async fn document(&self, ctx: &Context<'_>) -> Result<Option<DocumentObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            let document = ctx
                .content
                .documents
                .get_document(&self.metadata.id, self.metadata.version)
                .await?;
            Ok(document.map(DocumentObject::new))
        } else {
            Ok(None)
        }
    }

    async fn document_collaboration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<DocumentCollaborationObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
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
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        let document = ctx
            .content
            .documents
            .get_template(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(DocumentTemplateObject::new))
    }

    async fn guide(&self, ctx: &Context<'_>) -> Result<Option<GuideObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            let guide = ctx
                .content
                .guides
                .get_guide(&self.metadata.id, self.metadata.version)
                .await?;
            Ok(guide.map(GuideObject::new))
        } else {
            Ok(None)
        }
    }

    async fn guide_template(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<GuideTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
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
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Edit);
        ctx.metadata_permission_check(check).await?;
        let document = ctx
            .content
            .collection_templates
            .get_template(&self.metadata.id, self.metadata.version)
            .await?;
        Ok(document.map(CollectionTemplateObject::new))
    }

    async fn profiles(&self, ctx: &Context<'_>) -> Result<Vec<MetadataProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::View);
        ctx.metadata_permission_check(check).await?;
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
        let mut rels = Vec::new();
        for r in relationships {
            let include = if let Some(filter) = &filter {
                filter.contains(&r.relationship)
            } else {
                true
            };
            if !include {
                continue;
            }
            let check =
                PermissionCheck::new_with_metadata_id_advertised(r.id2, PermissionAction::View);
            if let Ok(metadata) = ctx.metadata_permission_check(check).await {
                rels.push(MetadataRelationshipObject::new(r, metadata));
            }
        }
        Ok(rels)
    }

    async fn supplementary(
        &self,
        ctx: &Context<'_>,
        key: Option<String>,
        plan_id: Option<String>,
    ) -> Result<Vec<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(key) = key {
            let check = PermissionCheck::new_with_metadata_supplementary(
                self.metadata.clone(),
                PermissionAction::View,
            );
            if ctx.metadata_permission_check(check).await.is_err() {
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
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::List);
        if ctx.metadata_permission_check(check).await.is_err() {
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
            let check = PermissionCheck::new_with_collection_id(id, PermissionAction::List);
            if let Ok(collection) = ctx.collection_permission_check(check).await {
                listable.push(collection.into());
            }
        }
        Ok(listable)
    }

    async fn comments(
        &self,
        ctx: &Context<'_>,
        pinned: Option<bool>,
        offset: i64,
        limit: i64,
    ) -> Result<CommentsObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
        let profile_id = profile.map(|p| p.id);
        let check =
            PermissionCheck::new_with_metadata(self.metadata.clone(), PermissionAction::Manage);
        let can_manage = ctx.metadata_permission_check(check).await.is_ok();
        if pinned.is_some() && pinned.unwrap() {
            let attribute = "pinned".to_string();
            let attribute_value = "true".to_string();
            let comments = ctx
                .content
                .comments
                .get_metadata_comments_by_attribute(
                    &profile_id,
                    &self.metadata.id,
                    &self.metadata.version,
                    &attribute,
                    &attribute_value,
                    can_manage,
                    offset,
                    limit,
                )
                .await?;
            let count = ctx
                .content
                .comments
                .get_metadata_comments_by_attribute_count(
                    &profile_id,
                    &self.metadata.id,
                    &self.metadata.version,
                    &attribute,
                    &attribute_value,
                    can_manage,
                )
                .await?;
            Ok(CommentsObject::new(can_manage, comments, count))
        } else {
            let comments = ctx
                .content
                .comments
                .get_metadata_comments(
                    &profile_id,
                    &self.metadata.id,
                    &self.metadata.version,
                    can_manage,
                    offset,
                    limit,
                )
                .await?;
            let count = ctx
                .content
                .comments
                .get_metadata_comments_count(
                    &profile_id,
                    &self.metadata.id,
                    &self.metadata.version,
                    can_manage,
                )
                .await?;
            Ok(CommentsObject::new(can_manage, comments, count))
        }
    }
}

impl From<Metadata> for MetadataObject {
    fn from(metadata: Metadata) -> Self {
        Self::new(metadata)
    }
}
