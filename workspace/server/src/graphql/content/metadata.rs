use std::borrow::ToOwned;
use crate::graphql::content::metadata_relationship::MetadataRelationshipObject;
use crate::graphql::content::permission::PermissionObject;
use crate::graphql::content::signed_url::SignedUrlObject;
use crate::graphql::content::supplementary::MetadataSupplementaryObject;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::metadata::{Metadata, MetadataType};
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::models::content::attributes_filter::AttributesFilterInput;

pub struct MetadataObject {
    metadata: Metadata,
}

pub struct MetadataWorkflowObject {
    metadata: Metadata,
}

pub struct MetadataContentObject {
    metadata: Metadata,
}

pub struct MetadataSourceObject {
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

    async fn parent_id(&self) -> Option<String> {
        self.metadata.parent_id.map(|id| id.to_string())
    }

    async fn version(&self) -> i32 {
        self.metadata.version
    }

    async fn trait_ids(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.get_metadata_trait_ids(&self.metadata.id).await
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

    async fn attributes(&self, filter: Option<AttributesFilterInput>) -> Value {
        if let Some(filter) = filter {
            return filter.filter(&self.metadata.attributes);
        }
        self.metadata.attributes.to_owned()
    }

    async fn item_attributes(&self) -> &Option<Value> {
        &self.metadata.item_attributes
    }

    async fn system_attributes(&self, ctx: &Context<'_>) -> Result<&Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        match ctx.check_metadata_action(&self.metadata.id, PermissionAction::Manage).await {
            Ok(_) => Ok(&self.metadata.system_attributes),
            Err(_) => Ok(&None)
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
        Ok(ctx.content
            .get_metadata_permissions(&self.metadata.id)
            .await?
            .into_iter()
            .map(Permission::into)
            .collect())
    }

    async fn relationships(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MetadataRelationshipObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .get_metadata_relationships(&self.metadata.id)
            .await?
            .into_iter()
            .map(|s| s.into())
            .collect())
    }

    async fn supplementary(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .get_metadata_supplementaries(&self.metadata.id)
            .await?
            .into_iter()
            .map(|s| MetadataSupplementaryObject::new(self.metadata.clone(), s))
            .collect())
    }

    async fn parent_collections(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let collections = ctx.content
            .get_metadata_parent_collection_ids(&self.metadata.id, offset, limit)
            .await?;
        let mut listable = Vec::new();
        for id in collections {
            if let Ok(collection) = ctx.check_collection_action(&id, PermissionAction::List).await {
                listable.push(collection.into());
            }
        }
        Ok(listable)
    }
}

pub struct MetadataContentUrls {
    metadata: Metadata,
}

#[Object(name = "MetadataContentUrls")]
impl MetadataContentUrls {
    async fn download(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_metadata_download_signed_url(&ctx.security, &ctx.principal, &self.metadata, None)
            .await?
            .into())
    }

    async fn upload(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_metadata_upload_signed_url(&ctx.security, &ctx.principal, &self.metadata, None)
            .await?
            .into())
    }
}

#[Object(name = "MetadataContent")]
impl MetadataContentObject {
    #[graphql(name = "type")]
    async fn content_type(&self) -> &String {
        &self.metadata.content_type
    }

    async fn length(&self) -> Option<i64> {
        self.metadata.content_length
    }

    async fn urls(&self) -> MetadataContentUrls {
        MetadataContentUrls {
            metadata: self.metadata.clone(),
        }
    }

    async fn text(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
        Ok(ctx.storage.get(&path).await?)
    }

    async fn json(&self, ctx: &Context<'_>) -> Result<Value, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
        let text = ctx.storage.get(&path).await?;
        Ok(serde_json::from_str(text.as_str())?)
    }
}

#[Object(name = "MetadataWorkflow")]
impl MetadataWorkflowObject {
    async fn state(&self) -> &String {
        &self.metadata.workflow_state_id
    }

    async fn pending(&self) -> &Option<String> {
        &self.metadata.workflow_state_pending_id
    }

    async fn delete_workflow(&self) -> &Option<String> {
        &self.metadata.delete_workflow_id
    }

    async fn plans(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let plans_ids = ctx.content.get_metadata_plans(&self.metadata.id).await?;
        let mut plans = Vec::<WorkflowExecutionPlanObject>::new();
        for (plan_id, queue) in plans_ids {
            let id = WorkflowExecutionId {
                id: plan_id.parse()?,
                queue,
            };
            let plan = ctx.workflow.get_execution_plan(&id).await?;
            if plan.is_none() {
                continue;
            }
            plans.push(plan.unwrap().into());
        }
        Ok(plans)
    }
}

#[Object(name = "MetadataSource")]
impl MetadataSourceObject {
    async fn id(&self) -> Option<String> {
        self.metadata.source_id.map(|s| s.to_string())
    }

    async fn identifier(&self) -> &Option<String> {
        &self.metadata.source_identifier
    }
}

impl From<Metadata> for MetadataObject {
    fn from(metadata: Metadata) -> Self {
        Self::new(metadata)
    }
}
