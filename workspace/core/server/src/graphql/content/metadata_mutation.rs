use crate::context::BoscaContext;
use crate::graphql::content::guide_step::GuideStepObject;
use crate::graphql::content::guide_step_module::GuideStepModuleObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::metadata_relationship::MetadataRelationshipObject;
use crate::graphql::content::permission::PermissionObject;
use crate::graphql::content::supplementary::MetadataSupplementaryObject;
use crate::graphql::workflows::workflow_execution_plan::WorkflowExecutionPlanObject;
use crate::models::content::collection::MetadataChildInput;
use crate::models::content::document::DocumentInput;
use crate::models::content::guide::GuideInput;
use crate::models::content::guide_step::GuideStepInput;
use crate::models::content::guide_step_module::GuideStepModuleInput;
use crate::models::content::guide_template_step::GuideTemplateStep;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;
use crate::models::content::metadata::{Metadata, MetadataInput};
use crate::models::content::metadata_profile::MetadataProfileInput;
use crate::models::content::metadata_relationship::MetadataRelationshipInput;
use crate::models::content::metadata_workflow_state::{
    MetadataWorkflowCompleteState, MetadataWorkflowState,
};
use crate::models::content::search::SearchDocumentInput;
use crate::models::content::supplementary::MetadataSupplementaryInput;
use crate::models::security::permission::{Permission, PermissionAction, PermissionInput};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::models::workflow::execution_plan::WorkflowExecutionPlan;
use crate::util::storage::{index_documents, storage_system_metadata_delete};
use async_graphql::*;
use bytes::Bytes;
use futures_util::AsyncReadExt;
use object_store::MultipartUpload;
use serde_json::json;
use uuid::Uuid;
use crate::workflow::core_workflows::METADATA_DELETE_FINALIZE;

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
        let mut metadatas = vec![MetadataChildInput {
            metadata: metadata.clone(),
            attributes: collection_item_attributes,
        }];
        let metadata_ids = ctx.content.metadata.add_all(ctx, &mut metadatas).await?;
        let Some((metadata_id, version, active_version)) = metadata_ids.first() else {
            return Err(Error::new("Error creating metadata"));
        };
        let new_metadata = if version == active_version {
            ctx.content.metadata.get(metadata_id).await?.unwrap()
        } else {
            ctx.content
                .metadata
                .get_by_version(metadata_id, *version)
                .await?
                .unwrap()
        };
        Ok(new_metadata.into())
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
        let template_id = Uuid::parse_str(template_id.as_str())?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        Ok(add_document_impl(
            ctx,
            &parent_collection_id,
            &template_id,
            template_version,
            "New Document",
        )
        .await?
        .map(MetadataObject::new))
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_guide_step(
        &self,
        ctx: &Context<'_>,
        parent_collection_id: String,
        metadata_id: String,
        version: i32,
        template_id: String,
        template_version: i32,
        template_step_id: i64,
        sort: i32,
    ) -> Result<GuideStepObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = Uuid::parse_str(parent_collection_id.as_str())?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, version, PermissionAction::View)
            .await?;
        let template = ctx
            .check_metadata_version_action(&template_id, template_version, PermissionAction::View)
            .await?;
        if template.content_type != "bosca/v-guide-step-template" {
            return Err(Error::new("invalid template"));
        }
        let template_step = ctx
            .content
            .guides
            .get_template_step(&template.id, template.version, template_step_id)
            .await?;
        if template_step.is_none() {
            return Err(Error::new("invalid step"));
        }
        let template_step = template_step.unwrap();
        let step = add_study_step_impl(
            ctx,
            &parent_collection_id,
            &template_id,
            template_version,
            0,
            &template_step,
        )
        .await?;
        let new_step = ctx
            .content
            .guides
            .add_guide_step(&metadata_id, version, &step, sort)
            .await?;
        Ok(GuideStepObject::new(new_step))
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_guide_step_module(
        &self,
        ctx: &Context<'_>,
        parent_collection_id: String,
        metadata_id: String,
        version: i32,
        template_id: String,
        template_version: i32,
        template_step_id: i64,
        template_module_id: i64,
        sort: i32,
    ) -> Result<GuideStepModuleObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = Uuid::parse_str(parent_collection_id.as_str())?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let metadata_id = Uuid::parse_str(&metadata_id)?;
        ctx.check_metadata_version_action(&metadata_id, version, PermissionAction::View)
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
        if template_module.is_none() {
            return Err(Error::new("invalid module"));
        }
        let template_module = template_module.unwrap();
        let module = add_study_module_impl(ctx, &parent_collection_id, 0, &template_module).await?;
        let new_module = ctx
            .content
            .guides
            .add_guide_step_module(&metadata_id, version, template_step_id, sort, &module)
            .await?;
        Ok(GuideStepModuleObject::new(new_module))
    }

    async fn add_guide(
        &self,
        ctx: &Context<'_>,
        parent_collection_id: String,
        template_id: String,
        template_version: i32,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let parent_collection_id = Uuid::parse_str(parent_collection_id.as_str())?;
        let template_id = Uuid::parse_str(template_id.as_str())?;
        ctx.check_collection_action(&parent_collection_id, PermissionAction::Edit)
            .await?;
        let template = ctx
            .check_metadata_version_action(&template_id, template_version, PermissionAction::View)
            .await?;
        if template.content_type != "bosca/v-guide-template" {
            return Err(Error::new("invalid template"));
        }
        let Some(template_document) = ctx
            .content
            .documents
            .get_template(&template.id, template.version)
            .await?
        else {
            return Err(Error::new("missing template"));
        };
        let Some(template_guide) = ctx
            .content
            .guides
            .get_template(&template.id, template.version)
            .await?
        else {
            return Err(Error::new("missing guide"));
        };
        let mut attrs = json!({
            "editor.type": "Guide",
        });
        if let Some(default_attributes) = &template_document.default_attributes {
            if let serde_json::Value::Object(ref mut attrs_obj) = attrs {
                if let serde_json::Value::Object(default_obj) = default_attributes.clone() {
                    attrs_obj.extend(default_obj.into_iter());
                }
            }
        }
        let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
        let categories = ctx
            .content
            .metadata
            .get_categories(&parent_collection_id)
            .await?;
        let template_steps = ctx
            .content
            .guides
            .get_template_steps(&template.id, template.version)
            .await?;
        let mut steps = Vec::new();
        for (index, template_step) in template_steps.iter().enumerate() {
            let step = add_study_step_impl(
                ctx,
                &parent_collection_id,
                &template_id,
                template_version,
                index,
                template_step,
            )
            .await?;
            steps.push(step);
        }
        let metadata = MetadataInput {
            parent_collection_id: Some(parent_collection_id.to_string()),
            category_ids: Some(categories.iter().map(|c| c.id.to_string()).collect()),
            name: "New Guide".to_string(),
            content_type: "bosca/v-guide".to_string(),
            language_tag: template.language_tag,
            attributes: Some(attrs),
            guide: Some(GuideInput {
                guide_type: template_guide.guide_type,
                rrule: template_guide.rrule.map(|rrule| rrule.to_string()),
                template_metadata_id: Some(template.id.to_string()),
                template_metadata_version: Some(template.version),
                steps,
            }),
            document: Some(DocumentInput {
                template_metadata_id: Some(template.id.to_string()),
                template_metadata_version: Some(template.version),
                title: "New Guide".to_string(),
                content: template_document.content.clone(),
            }),
            profiles: profile.map(|p| {
                vec![MetadataProfileInput {
                    profile_id: p.id.to_string(),
                    relationship: "author".to_string(),
                }]
            }),
            ..Default::default()
        };
        let mut add = vec![MetadataChildInput {
            metadata,
            attributes: None,
        }];
        let ids = ctx.content.metadata.add_all(ctx, &mut add).await?;
        let id = ids.first().unwrap();
        let metadata = ctx.content.metadata.get(&id.0).await?;
        Ok(metadata.map(MetadataObject::new))
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
        let mut metadatas = metadatas;
        let metadata_ids = ctx.content.metadata.add_all(ctx, &mut metadatas).await?;
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
        ctx.content.metadata.mark_deleted(&metadata.id).await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(METADATA_DELETE_FINALIZE.to_string()),
            metadata_id: Some(metadata.id),
            metadata_version: Some(metadata.version),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(true)
    }

    async fn permanently_delete(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_has_admin_account().await?;
        ctx.content.metadata.delete(ctx, &id).await?;
        Ok(true)
    }

    async fn delete_content(&self, ctx: &Context<'_>, metadata_id: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Delete)
            .await?;
        let storage_systems = ctx.workflow.get_storage_systems().await?;
        if metadata.uploaded.is_some() {
            storage_system_metadata_delete(&ctx.storage, &metadata, &storage_systems, &ctx.search)
                .await?;
            ctx.content.metadata.set_upload_removed(&id).await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn add_search_documents(
        &self,
        ctx: &Context<'_>,
        storage_system_id: String,
        documents: Vec<SearchDocumentInput>,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let storage_system_id = Uuid::parse_str(storage_system_id.as_str())?;
        let storage_system = ctx.workflow.get_storage_system(&storage_system_id).await?;
        if storage_system.is_none() {
            return Err(Error::new("invalid storage system"));
        }
        index_documents(ctx, &documents, storage_system.as_ref().unwrap()).await?;
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
        ctx.content.metadata.add_category(&id, &category_id).await?;
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
            .delete_category(&id, &category_id)
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
        ctx.content.metadata.add_trait(&id, &trait_id).await?;
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
        ctx.content.metadata.delete_trait(&id, &trait_id).await?;
        if t.is_some() && t.as_ref().unwrap().delete_workflow_id.is_some() {
            let mut request = EnqueueRequest {
                workflow_id: t.unwrap().delete_workflow_id.clone(),
                metadata_id: Some(metadata.id),
                metadata_version: Some(metadata.version),
                ..Default::default()
            };
            let plan = ctx
                .workflow
                .enqueue_workflow(ctx, &mut request)
                .await?;
            return Ok(plan.into_iter().map(WorkflowExecutionPlanObject::new).collect());
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
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.metadata.set_public(&id, public).await?;
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
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.metadata.set_public_content(&id, public).await?;
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
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .set_supplementary_public(&id, public)
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
            .add_metadata_permission(&permission)
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
            .delete_metadata_permission(&permission)
            .await?;
        Ok(permission.into())
    }

    async fn add_supplementary(
        &self,
        ctx: &Context<'_>,
        supplementary: MetadataSupplementaryInput,
    ) -> Result<MetadataSupplementaryObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(supplementary.metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .add_supplementary(&supplementary)
            .await?;
        match ctx
            .content
            .metadata
            .get_supplementary(&id, &supplementary.key)
            .await?
        {
            Some(supplementary) => Ok(MetadataSupplementaryObject::new(metadata, supplementary)),
            None => Err(Error::new("Error creating metadata")),
        }
    }

    async fn delete_supplementary(
        &self,
        ctx: &Context<'_>,
        id: String,
        key: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        ctx.check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content.metadata.delete_supplementary(&id, &key).await?;
        Ok(true)
    }

    async fn set_supplementary_uploaded(
        &self,
        ctx: &Context<'_>,
        metadata_id: String,
        supplementary_key: String,
        content_type: String,
        len: usize,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(metadata_id.as_str())?;
        ctx.check_metadata_action(&id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .set_supplementary_uploaded(&id, &supplementary_key, content_type.as_str(), len)
            .await?;
        Ok(true)
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
        ctx.content.metadata.add_relationship(&relationship).await?;
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
            .delete_relationship(&id1, &id2, &relationship)
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
                .set_metadata_workflow_state(
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
                .set_metadata_workflow_state(
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
        ctx.check_metadata_action(&metadata_id, PermissionAction::Manage)
            .await?;
        ctx.content
            .metadata
            .set_attributes(&metadata_id, attributes)
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
            .set_system_attributes(&metadata_id, attributes)
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
        let mut multipart = ctx.storage.put_multipart(&path).await?;
        let mut content = file.value(octx)?.into_async_read();
        let mut buf = vec![0_u8; 524288];
        let mut len = 0;
        loop {
            let read = content.read(&mut buf).await?;
            if read > 0 {
                len += read;
                let buf_slice = buf[..read].to_vec();
                multipart.put_part(buf_slice.into()).await?;
            } else {
                multipart.complete().await?;
                break;
            }
        }
        ctx.content
            .metadata
            .set_uploaded(&metadata_id, &None, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_supplementary_contents(
        &self,
        ctx: &Context<'_>,
        id: String,
        key: String,
        content_type: String,
        file: Upload,
    ) -> Result<bool, Error> {
        let octx = ctx;
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Manage)
            .await?;
        let path = ctx
            .storage
            .get_metadata_path(&metadata, Some(key.to_owned()))
            .await?;
        let mut multipart = ctx.storage.put_multipart(&path).await?;
        let mut content = file.value(octx)?.into_async_read();
        let mut buf = vec![0_u8; 524288];
        let mut len = 0;
        loop {
            let read = content.read(&mut buf).await?;
            if read > 0 {
                len += read;
                let buf_slice = buf[..read].to_vec();
                multipart.put_part(buf_slice.into()).await?;
            } else {
                multipart.complete().await?;
                break;
            }
        }
        ctx.content
            .metadata
            .set_supplementary_uploaded(&metadata_id, &key, &content_type, len)
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
            .set_uploaded(&metadata_id, &None, &content_type, len)
            .await?;
        Ok(true)
    }

    async fn set_supplementary_text_contents(
        &self,
        ctx: &Context<'_>,
        id: String,
        key: String,
        content_type: String,
        content: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata_id = Uuid::parse_str(id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id, PermissionAction::Edit)
            .await?;
        let path = ctx
            .storage
            .get_metadata_path(&metadata, Some(key.clone()))
            .await?;
        let bytes: Bytes = content.into();
        let len = bytes.len();
        ctx.storage.put(&path, bytes).await?;
        ctx.content
            .metadata
            .set_supplementary_uploaded(&metadata_id, &key, &content_type, len)
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
            .set_uploaded(&metadata_id, &None, &content_type, len)
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
            .set_uploaded(&metadata_id, &None, &content_type, len)
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
}

async fn add_study_step_impl(
    ctx: &BoscaContext,
    parent_collection_id: &Uuid,
    template_id: &Uuid,
    template_version: i32,
    step_index: usize,
    step: &GuideTemplateStep,
) -> Result<GuideStepInput, Error> {
    let new_metadata = if let Some(template_id) = step.template_metadata_id {
        if let Some(template_version) = step.template_metadata_version {
            let title = if step_index == 0 {
                "New Step".to_string()
            } else {
                format!("New Step {}", step_index + 1)
            };
            add_document_impl(
                ctx,
                parent_collection_id,
                &template_id,
                template_version,
                &title,
            )
            .await?
        } else {
            None
        }
    } else {
        None
    };

    let modules = ctx
        .content
        .guides
        .get_template_step_modules(template_id, template_version, step_index as i64)
        .await?;

    let mut new_modules = Vec::new();
    for (index, module) in modules.iter().enumerate() {
        let module = add_study_module_impl(ctx, parent_collection_id, index, module).await?;
        new_modules.push(module);
    }

    Ok(GuideStepInput {
        step_metadata_id: new_metadata.as_ref().map(|m| m.id.to_string()),
        step_metadata_version: new_metadata.map(|m| m.version),
        modules: new_modules,
    })
}

async fn add_study_module_impl(
    ctx: &BoscaContext,
    parent_collection_id: &Uuid,
    module_index: usize,
    module: &GuideTemplateStepModule,
) -> Result<GuideStepModuleInput, Error> {
    let title = if module_index == 0 {
        "New Module".to_string()
    } else {
        format!("New Module {}", module_index + 1)
    };
    let Some(metadata) = add_document_impl(
        ctx,
        parent_collection_id,
        &module.template_metadata_id,
        module.template_metadata_version,
        &title,
    )
    .await?
    else {
        return Err(Error::new("Error creating metadata"));
    };
    Ok(GuideStepModuleInput {
        module_metadata_id: metadata.id.to_string(),
        module_metadata_version: metadata.version,
    })
}

async fn add_document_impl(
    ctx: &BoscaContext,
    parent_collection_id: &Uuid,
    template_id: &Uuid,
    template_version: i32,
    title: &str,
) -> Result<Option<Metadata>, Error> {
    let template = ctx
        .check_metadata_version_action(template_id, template_version, PermissionAction::View)
        .await?;
    if template.content_type != "bosca/v-document-template" {
        return Err(Error::new("invalid template"));
    }
    let Some(template_document) = ctx
        .content
        .documents
        .get_template(&template.id, template.version)
        .await?
    else {
        return Err(Error::new("missing template"));
    };
    let editor_type = template
        .attributes
        .get("editor.type")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let template_type = template
        .attributes
        .get("template.type")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let content_type = format!("bosca/v-{}", template_type.to_lowercase());
    let mut attrs = json!({
        "editor.type": editor_type.to_string(),
    });
    if let Some(default_attributes) = &template_document.default_attributes {
        if let serde_json::Value::Object(ref mut attrs_obj) = attrs {
            if let serde_json::Value::Object(default_obj) = default_attributes.clone() {
                attrs_obj.extend(default_obj.into_iter());
            }
        }
    }
    let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
    let categories = ctx
        .content
        .metadata
        .get_categories(parent_collection_id)
        .await?;
    let metadata = MetadataInput {
        parent_collection_id: Some(parent_collection_id.to_string()),
        category_ids: Some(categories.iter().map(|c| c.id.to_string()).collect()),
        name: title.to_string(),
        content_type: content_type.to_string(),
        language_tag: template.language_tag,
        attributes: Some(attrs),
        document: Some(DocumentInput {
            template_metadata_id: Some(template.id.to_string()),
            template_metadata_version: Some(template.version),
            title: title.to_string(),
            content: template_document.content.clone(),
        }),
        profiles: profile.map(|p| {
            vec![MetadataProfileInput {
                profile_id: p.id.to_string(),
                relationship: "author".to_string(),
            }]
        }),
        ..Default::default()
    };
    let mut add = vec![MetadataChildInput {
        metadata,
        attributes: None,
    }];
    let ids = ctx.content.metadata.add_all(ctx, &mut add).await?;
    let id = ids.first().unwrap();
    ctx.content.metadata.get(&id.0).await
}
