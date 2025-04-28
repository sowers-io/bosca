use crate::context::BoscaContext;
use crate::graphql::cache::CacheObject;
use async_graphql::{Context, Error, Object};

pub struct CachesObject {}

#[Object(name = "CachesObject")]
impl CachesObject {
    async fn caches(&self, ctx: &Context<'_>) -> Result<Vec<CacheObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.cache
            .get_cache_names()
            .await
            .map(|c| c.into_iter().map(CacheObject::new).collect())
    }
}
