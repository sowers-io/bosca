use crate::models::content::attribute_type::AttributeType;
use crate::models::content::attribute_ui_type::AttributeUiType;
use crate::models::content::template_attribute::TemplateAttribute;
use crate::models::content::template_workflow::TemplateWorkflow;
use async_graphql::Object;
use serde_json::Value;
use crate::graphql::content::template_workflow::TemplateWorkflowObject;
use crate::models::content::attribute_location::AttributeLocation;

pub struct TemplateAttributeObject {
    pub attribute: TemplateAttribute,
    pub workflows: Vec<TemplateWorkflow>,
}

impl TemplateAttributeObject {
    pub fn new(attribute: TemplateAttribute, workflows: Vec<TemplateWorkflow>) -> Self {
        Self { attribute, workflows }
    }
}

#[Object(name = "TemplateAttribute")]
impl TemplateAttributeObject {
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
    
    pub async fn location(&self) -> &AttributeLocation {
        &self.attribute.location
    }

    pub async fn supplementary_key(&self) -> &Option<String> {
        &self.attribute.supplementary_key
    }

    pub async fn workflows(&self) -> Vec<TemplateWorkflowObject> {
        self.workflows.iter().cloned().map(TemplateWorkflowObject::new).collect()
    }
}
