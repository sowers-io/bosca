use crate::context::BoscaContext;
use crate::graphql::content::document_template::DocumentTemplateObject;
use async_graphql::{Context, Error, Object};

pub struct DocumentTemplatesObject {}

#[Object(name = "DocumentTemplates")]
impl DocumentTemplatesObject {
    pub async fn all(&self, ctx: &Context<'_>) -> Result<Vec<DocumentTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .documents
            .get_templates()
            .await?
            .into_iter()
            .map(DocumentTemplateObject::new)
            .collect())
    }
}
