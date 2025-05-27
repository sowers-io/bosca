use crate::context::BoscaContext;
use crate::graphql::content::guide_step::GuideStepObject;
use crate::graphql::content::guide_step_module::GuideStepModuleObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::metadata_relationship::MetadataRelationshipObject;
use crate::graphql::content::metadata_supplementary::MetadataSupplementaryObject;
use crate::graphql::content::permission::PermissionObject;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::bible::bible::BibleInput;
use crate::models::content::collection::MetadataChildInput;
use crate::models::content::document::DocumentInput;
use crate::models::content::metadata::MetadataInput;
use crate::models::content::metadata_relationship::MetadataRelationshipInput;
use crate::models::content::metadata_supplementary::MetadataSupplementaryInput;
use crate::models::content::metadata_workflow_state::{
    MetadataWorkflowCompleteState, MetadataWorkflowState,
};
use crate::models::security::permission::{Permission, PermissionAction, PermissionInput};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use crate::util::upload::upload_file;
use async_graphql::*;
use bytes::Bytes;
use chrono::{DateTime, Timelike, Utc};
use rrule::RRuleSet;
use uuid::Uuid;
use crate::graphql::content::metadata_template_mutation::MetadataTemplateMutationObject;

#[derive(InputObject, Clone, Debug, Default)]
pub struct WorkflowConfigurationInput {
    pub activity_id: String,
    pub configuration: serde_json::Value,
}

pub struct MetadataMutationObject {}

#[Object(name = "MetadataMutation")]
impl MetadataMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        metadata: MetadataInput,
        collection_item_attributes: Option<serde_json::Value>,
    ) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = match &metadata.parent_collection_id {
            Some(id) => Uuid::parse_str(id),
            None => Uuid::parse_str("00000000-0000-0000-0000-000000000000"),
        }?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let (metadata_id, version, active_version) = ctx
            .content
            .metadata
            .add(ctx, &metadata, collection_item_attributes)
            .await?;
        let new_metadata = if version == active_version {
            ctx.content.metadata.get(&metadata_id).await?.unwrap()
        } else {
            ctx.content
                .metadata
                .get_by_version(&metadata_id, version)
                .await?
                .unwrap()
        };
        Ok(new_metadata.into())
    }

    async fn template(&self, ctx: &Context<'_>, metadata_id: String, metadata_version: i32) -> Result<MetadataTemplateMutationObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&metadata_id)?;
        let metadata = ctx
            .check_metadata_version_action(&id, metadata_version, PermissionAction::Edit)
            .await?;
        Ok(MetadataTemplateMutationObject::new(metadata))
    }

    async fn add_document(
        &self,
        ctx: &Context<'_>,
        parent_collection_id: String,
        template_id: String,
        template_version: i32,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = Uuid::parse_str(parent_collection_id.as_str())?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        let permissions = ctx
            .content
            .collection_permissions
            .get(&parent_collection_id)
            .await?;
        let (id, _) = ctx
            .content
            .documents
            .add_document_from_template(
                ctx,
                Some(parent_collection_id),
                &template_id,
                template_version,
                "New Document",
                "bosca/v-document",
                &permissions,
            )
            .await?;
        let metadata = ctx.content.metadata.get(&id).await?;
        Ok(metadata.map(MetadataObject::new))
    }

    async fn add_guide(
        &self,
        ctx: &Context<'_>,
        parent_collection_id: String,
        template_id: String,
        template_version: i32,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = Uuid::parse_str(&parent_collection_id)?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let permissions = ctx
            .content
            .collection_permissions
            .get(&parent_collection_id)
            .await?;
        let template_id = Uuid::parse_str(&template_id)?;
        let (id, _) = ctx
            .content
            .guides
            .add_guide_from_template(
                ctx,
                &parent_collection_id,
                &template_id,
                template_version,
                &permissions,
            )
            .await?;
        let metadata = ctx.content.metadata.get(&id).await?;
        Ok(metadata.map(MetadataObject::new))
    }

    async fn set_guide_start_date(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
        date: DateTime<Utc>,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        let metadata = ctx
            .check_metadata_version_action(&metadata_id, metadata_version, PermissionAction::Edit)
            .await?;
        if let Some(guide) = ctx
            .content
            .guides
            .get_guide(&metadata_id, metadata_version)
            .await?
        {
            if let Some(rrule) = guide.rrule {
                let date = date
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap();
                let new_rrule = RRuleSet::new(date.with_timezone(&rrule::Tz::UTC))
                    .set_exdates(rrule.get_exdate().clone())
                    .set_rrules(rrule.get_rrule().clone())
                    .set_rdates(rrule.get_rdate().clone());
                ctx.content
                    .guides
                    .set_guide_rrule(ctx, &metadata_id, metadata_version, new_rrule)
                    .await?;
            }
        } else {
            return Err(Error::new("guide not found"));
        }
        if metadata.version == metadata_version {
            let metadata = ctx.content.metadata.get(&metadata_id).await?;
            Ok(metadata.map(MetadataObject::new))
        } else {
            let metadata = ctx
                .content
                .metadata
                .get_by_version(&metadata_id, metadata_version)
                .await?;
            Ok(metadata.map(MetadataObject::new))
        }
    }

    async fn delete_guide_step(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
        step_id: i64,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, metadata_version, PermissionAction::Delete)
            .await?;
        ctx.content
            .guides
            .delete_guide_step(ctx, &metadata_id, metadata_version, step_id)
            .await?;
        Ok(true)
    }

    async fn delete_guide(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, metadata_version, PermissionAction::Delete)
            .await?;
        ctx.content
            .guides
            .delete_guide(ctx, &metadata_id, metadata_version)
            .await?;
        Ok(true)
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_guide_step(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
        template_id: String,
        template_version: i32,
        template_step_id: i64,
        sort: i32,
    ) -> Result<GuideStepObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, metadata_version, PermissionAction::Edit)
            .await?;
        let permissions = ctx
            .content
            .metadata_permissions
            .get_metadata_permissions(&metadata_id)
            .await?;
        let template = ctx
            .check_metadata_version_action(&template_id, template_version, PermissionAction::View)
            .await?;
        if template.content_type != "bosca/v-document-template" {
            return Err(Error::new("invalid template"));
        }
        let Some(template_step) = ctx
            .content
            .guides
            .get_template_step(&template.id, template.version, template_step_id)
            .await?
        else {
            return Err(Error::new("invalid step"));
        };
        let Some(guide) = ctx
            .content
            .guides
            .get_guide(&metadata_id, metadata_version)
            .await?
        else {
            return Err(Error::new("guide not found"));
        };
        let step = ctx
            .content
            .guides
            .add_guide_step_from_template(
                ctx,
                &metadata_id,
                metadata_version,
                sort as usize,
                &template_step,
                &permissions,
            )
            .await?;
        Ok(if let Some(rrule) = guide.rrule.clone() {
            // TODO: cache this somewhere
            let recurrences = rrule.all((step.sort + 1) as u16);
            let date = recurrences
                .dates
                .into_iter()
                .map(|d| d.to_utc())
                .next_back();
            GuideStepObject::new(step, date)
        } else {
            GuideStepObject::new(step, None)
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_guide_step_module(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        metadata_version: i32,
        step_id: i64,
        template_id: String,
        template_version: i32,
        template_step_id: i64,
        template_module_id: i64,
        sort: i32,
    ) -> Result<GuideStepModuleObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, metadata_version, PermissionAction::Edit)
            .await?;
        let permissions = ctx
            .content
            .metadata_permissions
            .get_metadata_permissions(&metadata_id)
            .await?;
        let template = ctx
            .check_metadata_version_action(&template_id, template_version, PermissionAction::View)
            .await?;
        if template.content_type != "bosca/v-guide-module-template" {
            return Err(Error::new("invalid template"));
        }
        let template_module = ctx
            .content
            .guides
            .get_template_step_module(
                &template_id,
                template_version,
                template_step_id,
                template_module_id,
            )
            .await?;
        let Some(template_module) = template_module else {
            return Err(Error::new("invalid module"));
        };
        let module = ctx
            .content
            .guides
            .add_guide_module_from_template(
                ctx,
                &metadata_id,
                metadata_version,
                step_id,
                sort as usize,
                &template_module,
                &permissions,
            )
            .await?;
        Ok(GuideStepModuleObject::new(module))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        metadata: MetadataInput,
    ) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let current = ctx
            .check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        if current.workflow_state_id != "draft" && current.ready.is_some() {
            return Err(Error::new(
                "Cannot edit a non-draft metadata that has been marked ready",
            ));
        }
        ctx.content.metadata.edit(ctx, &id, &metadata).await?;
        match ctx.content.metadata.get(&id).await? {
            Some(metadata) => Ok(metadata.into()),
            None => Err(Error::new("Error creating metadata")),
        }
    }

    async fn add_bulk(
        &self,
        ctx: &Context<'_>,
        metadatas: Vec<MetadataChildInput>,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;

        let root_collection_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;

        for metadata in metadatas.iter() {
            let parent_collection_id = match &metadata.metadata.parent_collection_id {
                Some(id) => Uuid::parse_str(id)?,
                None => root_collection_id,
            };
            ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
                .await?;
        }

        let mut metadatas = metadatas;
        let metadata_ids = ctx
            .content
            .metadata
            .add_all(ctx, &mut metadatas, true)
            .await?;
        let mut metadatas = Vec::new();
        for (id, version, active_version) in metadata_ids {
            let metadata = if version == active_version {
                ctx.content.metadata.get(&id).await?.unwrap()
            } else {
                ctx.content
                    .metadata
                    .get_by_version(&id, version)
                    .await?
                    .unwrap()
            };
            metadatas.push(metadata.into());
        }
        Ok(metadatas)
    }

    async fn delete(&self, ctx: &Context<'_>, metadata_id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Delete)
            .await?;
        ctx.content.metadata.mark_deleted(ctx, &metadata.id).await?;
        Ok(true)
    }

    async fn permanently_delete(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_has_service_account().await?;
        ctx.content.metadata.delete(ctx, &id).await?;
        Ok(true)
    }

    async fn delete_content(&self, ctx: &Context<'_>, metadata_id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Delete)
            .await?;
        if metadata.uploaded.is_some() {
            ctx.content.metadata.set_upload_removed(ctx, &id).await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn set_categories(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        category_ids: Vec<String>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        let mut ids = Vec::new();
        for category_id in category_ids {
            let id = Uuid::parse_str(&category_id)?;
            ids.push(id);
        }
        ctx.content
            .metadata
            .set_categories(ctx, &id, &ids)
            .await?;
        Ok(true)
    }

    async fn add_category(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        category_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        let category_id = Uuid::parse_str(category_id.as_str())?;
        ctx.content
            .metadata
            .add_category(ctx, &id, &category_id)
            .await?;
        Ok(true)
    }

    async fn delete_category(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        category_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        let category_id = Uuid::parse_str(category_id.as_str())?;
        ctx.content
            .metadata
            .delete_category(ctx, &id, &category_id)
            .await?;
        Ok(true)
    }

    async fn add_trait(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        trait_id: String,
    ) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.metadata.add_trait(ctx, &id, &trait_id).await?;
        if metadata.ready.is_some() {
            let mut request = EnqueueRequest {
                trait_id: Some(trait_id),
                metadata_id: Some(metadata.id),
                metadata_version: Some(metadata.version),
                ..Default::default()
            };
            let plans = ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
            Ok(plans.into_iter().map(WorkflowExecutionPlan::into).collect())
        } else {
            Ok(vec![])
        }
    }

    async fn delete_trait(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        trait_id: String,
    ) -> Result<Vec<WorkflowExecutionPlanObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let t = ctx.workflow.get_trait(&trait_id).await?;
        if t.is_none() {
            return Ok(Vec::new());
        }
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .delete_trait(ctx, &id, &trait_id)
            .await?;
        if t.is_some() && t.as_ref().unwrap().delete_workflow_id.is_some() {
            let mut request = EnqueueRequest {
                workflow_id: t.unwrap().delete_workflow_id.clone(),
                metadata_id: Some(metadata.id),
                metadata_version: Some(metadata.version),
                ..Default::default()
            };
            let plan = ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
            return Ok(plan
                .into_iter()
                .map(WorkflowExecutionPlanObject::new)
                .collect());
        }
        Ok(Vec::new())
    }

    async fn set_public(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let mut metadata = ctx
            .check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        ctx.content.metadata.set_public(ctx, &id, public).await?;
        metadata.public = public;
        Ok(metadata.into())
    }

    async fn set_public_content(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let mut metadata = ctx
            .check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .set_public_content(ctx, &id, public)
            .await?;
        metadata.public = public;
        Ok(metadata.into())
    }

    async fn set_public_supplementary(
        &self,
        ctx: &Context<'_>,
        id: String,
        public: bool,
    ) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        let mut metadata = ctx
            .check_metadata_action(&id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata_supplementary
            .set_supplementary_public(ctx, &id, public)
            .await?;
        metadata.public = public;
        Ok(metadata.into())
    }

    async fn add_permission(
        &self,
        ctx: &Context<'_>,
        permission: PermissionInput,
    ) -> Result<PermissionObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let permission: Permission = permission.into();
        ctx.check_metadata_action(&permission.entity_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata_permissions
            .add_metadata_permission(ctx, &permission)
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
        ctx.check_metadata_action(&permission.entity_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata_permissions
            .delete_metadata_permission(ctx, &permission)
            .await?;
        Ok(permission.into())
    }

    async fn add_relationship(
        &self,
        ctx: &Context<'_>,
        relationship: MetadataRelationshipInput,
    ) -> Result<MetadataRelationshipObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id1 = Uuid::parse_str(relationship.id1.as_str())?;
        ctx.check_metadata_action(&id1, PermissionAction::Edit)
            .await?;
        let id2 = Uuid::parse_str(relationship.id2.as_str())?;
        ctx.check_metadata_action(&id2, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .add_relationship(ctx, &relationship)
            .await?;
        match ctx.content.metadata.get_relationship(&id1, &id2).await? {
            Some(relationship) => Ok(relationship.into()),
            None => Err(Error::new("error creating relationship")),
        }
    }

    async fn edit_relationship(
        &self,
        ctx: &Context<'_>,
        relationship: MetadataRelationshipInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id1 = Uuid::parse_str(relationship.id1.as_str())?;
        ctx.check_metadata_action(&id1, PermissionAction::Edit)
            .await?;
        let id2 = Uuid::parse_str(relationship.id2.as_str())?;
        ctx.check_metadata_action(&id2, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .edit_relationship(
                ctx,
                &id1,
                &id2,
                &relationship.relationship,
                &relationship.attributes,
            )
            .await?;
        Ok(true)
    }

    async fn delete_relationship(
        &self,
        ctx: &Context<'_>,
        id1: String,
        id2: String,
        relationship: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id1 = Uuid::parse_str(id1.as_str())?;
        ctx.check_metadata_action(&id1, PermissionAction::Edit)
            .await?;
        let id2 = Uuid::parse_str(id2.as_str())?;
        ctx.check_metadata_action(&id2, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .delete_relationship(ctx, &id1, &id2, &relationship)
            .await?;
        Ok(true)
    }

    async fn set_workflow_state(
        &self,
        ctx: &Context<'_>,
        state: MetadataWorkflowState,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(state.metadata_id.as_str())?;
        ctx.check_has_service_account().await?;
        if let Some(metadata) = ctx.content.metadata.get(&id).await? {
            ctx.content
                .metadata_workflows
                .set_state(
                    ctx,
                    &ctx.principal,
                    &metadata,
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
        state: MetadataWorkflowCompleteState,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(state.metadata_id.as_str())?;
        ctx.check_has_service_account().await?;
        if let Some(metadata) = ctx.content.metadata.get(&id).await? {
            let mut state_id = metadata.workflow_state_id.clone();
            if metadata.workflow_state_pending_id.is_some() {
                state_id = metadata.workflow_state_pending_id.clone().unwrap();
            }
            ctx.content
                .metadata_workflows
                .set_state(
                    ctx,
                    &ctx.principal,
                    &metadata,
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

    async fn set_metadata_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .set_attributes(ctx, &metadata_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_metadata_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .merge_attributes(ctx, &metadata_id, attributes)
            .await?;
        Ok(true)
    }

    async fn merge_metadata_relationship_attributes(
        &self,
        ctx: &Context<'_>,
        metadata1_id: String,
        metadata2_id: String,
        relationship: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata1_id = Uuid::parse_str(metadata1_id.as_str())?;
        let metadata2_id = Uuid::parse_str(metadata2_id.as_str())?;
        ctx.check_metadata_action(&metadata1_id, PermissionAction::Edit)
            .await?;
        ctx.check_metadata_action(&metadata2_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .merge_relationship_attributes(
                ctx,
                &metadata1_id,
                &metadata2_id,
                &relationship,
                attributes,
            )
            .await?;
        Ok(true)
    }

    async fn set_metadata_system_attributes(
        &self,
        ctx: &Context<'_>,
        id: String,
        attributes: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        ctx.check_metadata_action(&metadata_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .set_system_attributes(ctx, &metadata_id, attributes)
            .await?;
        Ok(true)
    }

    async fn set_metadata_contents(
        &self,
        ctx: &Context<'_>,
        id: String,
        content_type: Option<String>,
        file: Upload,
    ) -> Result<bool, Error> {
        let octx = ctx;
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        let path = ctx.storage.get_metadata_path(&metadata, None).await?;
        let len = upload_file(ctx, octx, path, file).await?;
        ctx.content
            .metadata
            .set_uploaded(ctx, &metadata_id, &None, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_metadata_text_contents(
        &self,
        ctx: &Context<'_>,
        id: String,
        content_type: Option<String>,
        content: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        let path = ctx.storage.get_metadata_path(&metadata, None).await?;
        let bytes: Bytes = content.into();
        let len = bytes.len();
        ctx.storage.put(&path, bytes).await?;
        ctx.content
            .metadata
            .set_uploaded(ctx, &metadata_id, &None, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_metadata_json_contents(
        &self,
        ctx: &Context<'_>,
        id: String,
        content_type: Option<String>,
        content: serde_json::Value,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        let path = ctx.storage.get_metadata_path(&metadata, None).await?;
        let content = content.to_string();
        let bytes: Bytes = content.into();
        let len = bytes.len();
        ctx.storage.put(&path, bytes).await?;
        ctx.content
            .metadata
            .set_uploaded(ctx, &metadata_id, &None, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_metadata_uploaded(
        &self,
        ctx: &Context<'_>,
        id: String,
        content_type: Option<String>,
        len: usize,
        ready: Option<bool>,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        ctx.content
            .metadata
            .set_uploaded(ctx, &metadata_id, &None, &content_type, len)
            .await?;
        if ready.is_some()
            && ready.unwrap()
            && metadata.ready.is_none()
            && !ctx
                .content
                .metadata_workflows
                .set_metadata_ready_and_enqueue(ctx, &metadata, configurations)
                .await?
        {
            return Ok(false);
        }
        Ok(true)
    }

    async fn set_metadata_ready(
        &self,
        ctx: &Context<'_>,
        id: String,
        configurations: Option<Vec<WorkflowConfigurationInput>>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        if metadata.ready.is_some() {
            return Ok(false);
        }
        ctx.content
            .metadata_workflows
            .set_metadata_ready_and_enqueue(ctx, &metadata, configurations)
            .await
    }

    async fn set_metadata_bible(
        &self,
        ctx: &Context<'_>,
        id: String,
        version: i32,
        bible: BibleInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_version_action(&metadata_id, version, PermissionAction::Manage)
            .await?;
        ctx.content
            .bibles
            .set_bible(&metadata.id, metadata.version, &bible)
            .await?;
        Ok(true)
    }

    async fn set_metadata_document(
        &self,
        ctx: &Context<'_>,
        id: String,
        version: i32,
        document: DocumentInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_version_action(&metadata_id, version, PermissionAction::Edit)
            .await?;
        ctx.content
            .documents
            .set_document(&metadata.id, metadata.version, &document)
            .await?;
        Ok(true)
    }

    async fn add_supplementary(
        &self,
        ctx: &Context<'_>,
        supplementary: MetadataSupplementaryInput,
    ) -> Result<MetadataSupplementaryObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(supplementary.metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Manage)
            .await?;
        let id = ctx
            .content
            .metadata_supplementary
            .add_supplementary(ctx, &supplementary)
            .await?;
        match ctx
            .content
            .metadata_supplementary
            .get_supplementary(&id)
            .await?
        {
            Some(supplementary) => Ok(MetadataSupplementaryObject::new(metadata, supplementary)),
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
        let supplementary_id = Uuid::parse_str(supplementary_id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        ctx.check_metadata_action(&supplementary.metadata_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata_supplementary
            .set_supplementary_uploaded(ctx, &supplementary_id, content_type.as_str(), len)
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
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let metadata = ctx
            .check_metadata_action(&supplementary.metadata_id, PermissionAction::Manage)
            .await?;
        let path = ctx
            .storage
            .get_metadata_path(&metadata, Some(supplementary_id))
            .await?;
        let len = upload_file(ctx, octx, path, file).await?;
        ctx.content
            .metadata_supplementary
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
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let metadata = ctx
            .check_metadata_action(&supplementary.metadata_id, PermissionAction::Manage)
            .await?;
        let path = ctx
            .storage
            .get_metadata_path(&metadata, Some(supplementary_id))
            .await?;
        let bytes: Bytes = content.into();
        let len = bytes.len();
        ctx.storage.put(&path, bytes).await?;
        ctx.content
            .metadata_supplementary
            .set_supplementary_uploaded(ctx, &supplementary_id, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn delete_supplementary(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::parse_str(id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let metadata = ctx
            .check_metadata_action(&supplementary.metadata_id, PermissionAction::Manage)
            .await?;
        let path = ctx
            .storage
            .get_metadata_path(&metadata, Some(supplementary_id))
            .await?;
        ctx.storage.delete(&path).await?;
        ctx.content
            .metadata_supplementary
            .delete_supplementary(ctx, &supplementary_id)
            .await?;
        Ok(true)
    }

    async fn detach_supplementary(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::parse_str(id.as_str())?;
        let Some(supplementary) = ctx
            .content
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
        else {
            return Err(Error::new("Supplementary not found"));
        };
        let metadata = ctx
            .check_metadata_action(&supplementary.metadata_id, PermissionAction::Manage)
            .await?;
        ctx.check_metadata_action(&metadata.id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata_supplementary
            .detach_supplementary(ctx, &supplementary_id)
            .await?;
        Ok(true)
    }
}
