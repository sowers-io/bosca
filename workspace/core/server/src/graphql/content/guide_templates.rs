use crate::context::BoscaContext;
use async_graphql::{Context, Error, Object};
use crate::graphql::content::guide_template::GuideTemplateObject;

pub struct GuideTemplatesObject {}

#[Object(name = "GuideTemplates")]
impl GuideTemplatesObject {
    pub async fn all(&self, ctx: &Context<'_>) -> Result<Vec<GuideTemplateObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .guides
            .get_templates()
            .await?
            .into_iter()
            .map(GuideTemplateObject::new)
            .collect())
    }
}
