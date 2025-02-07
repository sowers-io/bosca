use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::permission::PermissionObject;
use crate::models::content::collection::{Collection, CollectionType};
use async_graphql::{Context, Error, Object, Union};
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::attributes_filter::AttributesFilterInput;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::execution_plan::WorkflowExecutionId;

#[derive(Union)]
enum CollectionItem {
    Metadata(MetadataObject),
    Collection(CollectionObject),
}

pub struct CollectionObject {
    collection: Collection,
}

pub struct CollectionWorkflowObject<'a> {
    collection: &'a Collection,
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

    async fn slug(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.get_collection_slug(&self.collection.id).await
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

    async fn trait_ids(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.get_collection_trait_ids(&self.collection.id).await
    }

    async fn labels(&self) -> &Vec<String> {
        &self.collection.labels
    }

    async fn attributes(&self, filter: Option<AttributesFilterInput>) -> Value {
        if let Some(filter) = filter {
            return filter.filter(&self.collection.attributes);
        }
        self.collection.attributes.to_owned()
    }

    async fn item_attributes(&self) -> &Option<Value> {
        &self.collection.item_attributes
    }

    async fn system_attributes(&self) -> &Option<Value> {
        &self.collection.system_attributes
    }

    async fn ordering(&self) -> &Option<Value> {
        &self.collection.ordering
    }

    async fn created(&self) -> &DateTime<Utc> {
        &self.collection.created
    }

    async fn modified(&self) -> &DateTime<Utc> {
        &self.collection.modified
    }

    async fn parent_collections(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        Ok(ctx.content
            .get_collection_parent_collections(&self.collection.id, offset, limit)
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
        let items = ctx.content.get_collection_children(&self.collection, offset, limit).await?;
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

    async fn collections(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_collection_action(&self.collection.id, PermissionAction::List).await?;
        Ok(ctx.content
            .get_collection_child_collections(&self.collection, offset, limit)
            .await?
            .into_iter()
            .map(CollectionObject::new)
            .collect())
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
            .get_collection_child_metadata(&self.collection, offset, limit)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
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
            .get_collection_permissions(&self.collection.id)
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

#[Object(name = "CollectionWorkflow")]
impl CollectionWorkflowObject<'_> {
    async fn state(&self) -> &String {
        &self.collection.workflow_state_id
    }

    async fn pending(&self) -> &Option<String> {
        &self.collection.workflow_state_pending_id
    }

    async fn delete_workflow(&self) -> &Option<String> {
        &self.collection.delete_workflow_id
    }

    async fn plans(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let plans_ids = ctx.content.get_collection_plans(&self.collection.id).await?;
        let mut plans = Vec::<WorkflowExecutionPlanObject>::new();
        for (plan_id, queue) in plans_ids {
            let id = WorkflowExecutionId {
                id: plan_id,
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

impl From<Collection> for CollectionObject {
    fn from(collection: Collection) -> Self {
        Self::new(collection)
    }
}
