use crate::context::BoscaContext;
use crate::graphql::content::category_mutation::CategoryMutationObject;
use crate::graphql::content::collection_mutation::CollectionMutationObject;
use crate::graphql::content::metadata_mutation::MetadataMutationObject;
use crate::graphql::content::source_mutation::SourceMutationObject;
use async_graphql::{Context, Error, Object};

pub struct ContentMutationObject {}

#[Object(name = "ContentMutation")]
impl ContentMutationObject {
    async fn category(&self) -> CategoryMutationObject {
        CategoryMutationObject {}
    }
    async fn collection(&self) -> CollectionMutationObject {
        CollectionMutationObject {}
    }
    async fn metadata(&self) -> MetadataMutationObject {
        MetadataMutationObject {}
    }
    async fn sources(&self) -> SourceMutationObject {
        SourceMutationObject {}
    }

    async fn reindex(&self, ctx: &Context<'_>) -> async_graphql::Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        const LIMIT: i64 = 100;
        let mut offset = 0;
        loop {
            let items = ctx.content.collections.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                ctx.content.collections.index_collection(ctx, &item.id).await?;
            }
        }
        offset = 0;
        loop {
            let items = ctx.content.metadata.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                ctx.content.metadata.index_metadata(ctx, &item.id, Some(item.version)).await?;
            }
        }
        offset = 0;
        loop {
            let items = ctx.profile.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                ctx.profile.index_profile(ctx, &item.id).await?;
            }
        }
        Ok(true)
    }
}
