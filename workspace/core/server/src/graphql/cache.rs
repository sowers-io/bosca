use crate::context::BoscaContext;
use async_graphql::{Context, Error, Object};

pub struct CacheObject {
    name: String,
}

impl CacheObject {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[Object(name = "CacheObject")]
impl CacheObject {
    async fn name(&self) -> &String {
        &self.name
    }

    async fn keys(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.cache.get_cache_keys(&self.name).await
    }
}
