use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::datastores::persisted_queries::PersistedQueryInput;

pub struct PersistedQueriesMutationObject {
}

#[Object(name = "PersistedQueriesMutation")]
impl PersistedQueriesMutationObject {

    async fn add_all(&self, ctx: &Context<'_>, queries: Vec<PersistedQueryInput>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.add_queries(&queries).await?;
        Ok(true)
    }

    async fn delete_all(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.delete_queries().await?;
        Ok(true)
    }
    
    async fn add(&self, ctx: &Context<'_>, sha256: String, query: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.add_query(&sha256, &query).await?;
        Ok(true)
    }

    async fn delete(&self, ctx: &Context<'_>, sha256: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.delete_query(&sha256).await?;
        Ok(true)
    }
}