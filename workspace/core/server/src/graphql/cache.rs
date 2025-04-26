use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;

pub struct CacheObject {
}

#[Object(name = "CacheObject")]
impl CacheObject {

    async fn caches(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.cache.get_cache_names().await
    }

    async fn keys(&self, ctx: &Context<'_>, cache: String) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.cache.get_cache_keys(&cache).await
    }

    async fn remote_keys(&self, ctx: &Context<'_>, cache: String) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.cache.get_cache_remote_keys(&cache).await
    }
}