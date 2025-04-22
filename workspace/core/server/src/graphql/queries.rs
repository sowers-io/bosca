use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::datastores::persisted_queries::PersistedQuery;

pub struct PersistedQueriesObject {
}

#[Object(name = "PersistedQueriesObject")]
impl PersistedQueriesObject {

    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<PersistedQuery>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal_groups.contains(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.get_queries().await
    }
    
    async fn query(&self, ctx: &Context<'_>, sha256: String) -> Result<Option<PersistedQuery>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal_groups.contains(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        ctx.queries.get_query(&sha256).await
    }
}