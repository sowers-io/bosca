use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::content::category::CategoryObject;

pub struct CategoriesObject {
}

#[Object(name = "Categories")]
impl CategoriesObject {

    pub async fn all(&self, ctx: &Context<'_>) -> Result<Vec<CategoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let categories = ctx.content.categories.get_all().await?;
        Ok(categories.into_iter().map(CategoryObject::new).collect())
    }
}
