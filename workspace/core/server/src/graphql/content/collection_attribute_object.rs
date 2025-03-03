use crate::context::BoscaContext;
use crate::models::content::attribute_type::AttributeType;
use crate::models::content::attribute_ui_type::AttributeUiType;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;
use crate::graphql::content::collection_attribute_workflow_object::CollectionTemplateAttributeWorkflowObject;
use crate::models::content::template_attribute::TemplateAttribute;

pub struct CollectionTemplateAttributeObject {
    pub metadata_id: Uuid,
    pub version: i32,
    pub attribute: TemplateAttribute,
}

impl CollectionTemplateAttributeObject {
    pub fn new(metadata_id: Uuid, version: i32, attribute: TemplateAttribute) -> Self {
        Self {
            metadata_id,
            version,
            attribute,
        }
    }
}

#[Object(name = "CollectionTemplateAttribute")]
impl CollectionTemplateAttributeObject {
    pub async fn key(&self) -> &String {
        &self.attribute.key
    }

    pub async fn name(&self) -> &String {
        &self.attribute.name
    }

    pub async fn description(&self) -> &String {
        &self.attribute.description
    }

    pub async fn configuration(&self) -> &Option<Value> {
        &self.attribute.configuration
    }

    #[graphql(name = "type")]
    pub async fn attribute_type(&self) -> AttributeType {
        self.attribute.attribute_type
    }

    #[graphql(name = "ui")]
    pub async fn ui(&self) -> AttributeUiType {
        self.attribute.ui
    }

    pub async fn list(&self) -> bool {
        self.attribute.list
    }

    pub async fn supplementary_key(&self) -> &Option<String> {
        &self.attribute.supplementary_key
    }

    pub async fn workflows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CollectionTemplateAttributeWorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collection_templates
            .get_template_attribute_workflows(&self.metadata_id, self.version, &self.attribute.key)
            .await?
            .into_iter()
            .map(CollectionTemplateAttributeWorkflowObject::new)
            .collect())
    }
}
