use crate::graphql::content::signed_url::SignedUrlObject;
use crate::models::content::metadata::Metadata;
use crate::models::content::metadata_supplementary::MetadataSupplementary;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;
use crate::context::BoscaContext;

pub struct MetadataSupplementaryObject {
    metadata: Metadata,
    supplementary: MetadataSupplementary,
}

impl MetadataSupplementaryObject {
    pub fn new(metadata: Metadata, supplementary: MetadataSupplementary) -> Self {
        Self {
            metadata,
            supplementary,
        }
    }
}

pub struct MetadataSupplementaryContentObject {
    metadata: Metadata,
    supplementary: MetadataSupplementary,
}

pub struct MetadataSupplementarySourceObject {
    supplementary: MetadataSupplementary,
}

pub struct MetadataSupplementaryContentUrls {
    metadata: Metadata,
    supplementary: MetadataSupplementary,
}

#[Object(name = "MetadataSupplementaryContentUrls")]
impl MetadataSupplementaryContentUrls {
    async fn download(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_metadata_download_signed_url(
                &ctx.security,
                &ctx.principal,
                &self.metadata,
                Some(self.supplementary.id),
            )
            .await?
            .into())
    }

    async fn upload(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_metadata_upload_signed_url(
                &ctx.security,
                &ctx.principal,
                &self.metadata,
                Some(self.supplementary.id),
            )
            .await?
            .into())
    }
}

#[Object(name = "MetadataSupplementaryContent")]
impl MetadataSupplementaryContentObject {

    #[graphql(name = "type")]
    async fn content_type(&self) -> &String {
        &self.supplementary.content_type
    }

    async fn length(&self) -> Option<i64> {
        self.supplementary.content_length
    }

    async fn urls(&self) -> MetadataSupplementaryContentUrls {
        MetadataSupplementaryContentUrls {
            metadata: self.metadata.clone(),
            supplementary: self.supplementary.clone(),
        }
    }

    async fn text(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, Some(self.supplementary.id)).await?;
        Ok(ctx.storage.get(&path).await?)
    }

    async fn json(&self, ctx: &Context<'_>) -> Result<Value, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, Some(self.supplementary.id)).await?;
        let text = ctx.storage.get(&path).await?;
        Ok(serde_json::from_str(text.as_str())?)
    }
}

#[Object(name = "MetadataSupplementarySource")]
impl MetadataSupplementarySourceObject {
    async fn id(&self) -> String {
        self.supplementary
            .source_id
            .unwrap_or(Uuid::nil())
            .to_string()
    }

    async fn identifier(&self) -> &Option<String> {
        &self.supplementary.source_identifier
    }
}

#[Object(name = "MetadataSupplementary")]
impl MetadataSupplementaryObject {
    async fn id(&self) -> String {
        self.supplementary.id.to_string()
    }

    async fn plan_id(&self) -> Option<String> { self.supplementary.plan_id.map(|id| id.to_string()) }

    async fn metadata_id(&self) -> String {
        self.metadata.id.to_string()
    }

    async fn key(&self) -> &String {
        &self.supplementary.key
    }

    async fn name(&self) -> &String {
        &self.supplementary.name
    }

    async fn created(&self) -> String {
        self.supplementary.created.to_string()
    }

    async fn modified(&self) -> String {
        self.supplementary.modified.to_string()
    }

    async fn attributes(&self) -> &Option<Value> {
        &self.supplementary.attributes
    }

    async fn uploaded(&self) -> Option<String> {
        if self.supplementary.uploaded.is_some() {
            Some(self.supplementary.uploaded.unwrap().to_string())
        } else {
            None
        }
    }

    async fn content(&self) -> MetadataSupplementaryContentObject {
        MetadataSupplementaryContentObject {
            metadata: self.metadata.clone(),
            supplementary: self.supplementary.clone(),
        }
    }

    async fn source(&self) -> MetadataSupplementarySourceObject {
        MetadataSupplementarySourceObject {
            supplementary: self.supplementary.clone(),
        }
    }
}
