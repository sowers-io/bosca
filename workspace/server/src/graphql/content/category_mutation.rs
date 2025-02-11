use crate::context::BoscaContext;
use crate::graphql::content::category::CategoryObject;
use crate::models::content::category::{Category, CategoryInput};
use async_graphql::*;
use uuid::Uuid;

pub struct CategoryMutationObject {}

#[Object(name = "CategoryMutation")]
impl CategoryMutationObject {

    async fn add(
        &self,
        ctx: &Context<'_>,
        category: CategoryInput,
    ) -> Result<CategoryObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.content.categories.add(&category).await?;
        let category = Category {
            id,
            name: category.name.clone()
        };
        Ok(CategoryObject::new(category))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: String,
        category: CategoryInput,
    ) -> Result<CategoryObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        ctx.content.categories.edit(&id, &category).await?;
        let category = Category {
            id,
            name: category.name.clone()
        };
        Ok(CategoryObject::new(category))
    }

    async fn delete(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(id.as_str())?;
        ctx.content.categories.delete(&id).await?;
        Ok(true)
    }
}
