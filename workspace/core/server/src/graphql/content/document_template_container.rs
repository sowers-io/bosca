use crate::context::BoscaContext;
use crate::graphql::content::template_workflow::TemplateWorkflowObject;
use crate::models::content::document_template_container::DocumentTemplateContainer;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct DocumentTemplateContainerObject {
    pub metadata_id: Uuid,
    pub metadata_version: i32,
    pub container: DocumentTemplateContainer,
}

impl DocumentTemplateContainerObject {
    pub fn new(
        metadata_id: Uuid,
        metadata_version: i32,
        container: DocumentTemplateContainer,
    ) -> Self {
        Self {
            metadata_id,
            metadata_version,
            container,
        }
    }
}

#[Object(name = "DocumentTemplateContainer")]
impl DocumentTemplateContainerObject {
    pub async fn id(&self) -> &String {
        &self.container.id
    }

    pub async fn name(&self) -> &String {
        &self.container.name
    }

    pub async fn description(&self) -> &String {
        &self.container.description
    }

    pub async fn workflows(&self, ctx: &Context<'_>) -> Result<Vec<TemplateWorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_container_template_workflows(
                &self.metadata_id,
                self.metadata_version,
                &self.container.id,
            )
            .await?
            .into_iter()
            .map(TemplateWorkflowObject::new)
            .collect())
    }
}
