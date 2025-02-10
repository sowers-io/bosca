use crate::context::BoscaContext;
use crate::graphql::content::document_template_attribute_workflow_object::DocumentTemplateAttributeWorkflowObject;
use crate::models::content::document_attribute_type::DocumentAttributeType;
use crate::models::content::document_template_attributes::DocumentTemplateAttribute;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct DocumentTemplateAttributeObject {
    pub metadata_id: Uuid,
    pub version: i32,
    pub attribute: DocumentTemplateAttribute,
}

impl DocumentTemplateAttributeObject {
    pub fn new(metadata_id: Uuid, version: i32, attribute: DocumentTemplateAttribute) -> Self {
        Self {
            metadata_id,
            version,
            attribute,
        }
    }
}

#[Object(name = "DocumentTemplateAttribute")]
impl DocumentTemplateAttributeObject {
    pub async fn id(&self) -> i64 {
        self.attribute.id
    }

    pub async fn key(&self) -> &String {
        &self.attribute.key
    }

    pub async fn name(&self) -> &String {
        &self.attribute.name
    }

    pub async fn description(&self) -> &String {
        &self.attribute.description
    }

    #[graphql(name = "type")]
    pub async fn attribute_type(&self) -> DocumentAttributeType {
        self.attribute.attribute_type
    }

    pub async fn workflows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DocumentTemplateAttributeWorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_template_attribute_workflows(&self.metadata_id, self.version, &self.attribute.key)
            .await?
            .into_iter()
            .map(DocumentTemplateAttributeWorkflowObject::new)
            .collect())
    }
}
