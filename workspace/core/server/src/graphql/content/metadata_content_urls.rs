use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::content::signed_url::SignedUrlObject;
use crate::models::content::metadata::Metadata;

pub struct MetadataContentUrls {
    pub metadata: Metadata,
}

#[Object(name = "MetadataContentUrls")]
impl MetadataContentUrls {
    async fn download(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .storage
            .get_metadata_download_signed_url(&ctx.security, &ctx.principal, &self.metadata, None)
            .await?
            .into())
    }

    async fn upload(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .storage
            .get_metadata_upload_signed_url(&ctx.security, &ctx.principal, &self.metadata, None)
            .await?
            .into())
    }
}
