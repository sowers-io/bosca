use crate::context::BoscaContext;
use crate::graphql::content::collection_template::CollectionTemplateObject;
use async_graphql::{Context, Error, Object};

pub struct CollectionTemplatesObject {}

#[Object(name = "CollectionTemplates")]
impl CollectionTemplatesObject {
    pub async fn all(&self, ctx: &Context<'_>) -> Result<Vec<CollectionTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collection_templates
            .get_templates()
            .await?
            .into_iter()
            .map(|t| CollectionTemplateObject::new(t))
            .collect())
    }
}
