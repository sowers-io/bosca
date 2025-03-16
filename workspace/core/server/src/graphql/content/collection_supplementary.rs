use crate::graphql::content::signed_url::SignedUrlObject;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::models::content::collection::Collection;
use crate::models::content::collection_supplementary::CollectionSupplementary;

pub struct CollectionSupplementaryObject {
    collection: Collection,
    supplementary: CollectionSupplementary,
}

impl CollectionSupplementaryObject {
    pub fn new(collection: Collection, supplementary: CollectionSupplementary) -> Self {
        Self {
            collection,
            supplementary,
        }
    }
}

pub struct CollectionSupplementaryContentObject {
    collection: Collection,
    supplementary: CollectionSupplementary,
}

pub struct CollectionSupplementarySourceObject {
    supplementary: CollectionSupplementary,
}

pub struct CollectionSupplementaryContentUrls {
    collection: Collection,
    supplementary: CollectionSupplementary,
}

#[Object(name = "CollectionSupplementaryContentUrls")]
impl CollectionSupplementaryContentUrls {
    async fn download(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_collection_download_signed_url(
                &ctx.security,
                &ctx.principal,
                &self.collection,
                self.supplementary.key.clone(),
            )
            .await?
            .into())
    }

    async fn upload(&self, ctx: &Context<'_>) -> Result<SignedUrlObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.storage
            .get_collection_upload_signed_url(
                &ctx.security,
                &ctx.principal,
                &self.collection,
                self.supplementary.key.clone(),
            )
            .await?
            .into())
    }
}

#[Object(name = "CollectionSupplementaryContent")]
impl CollectionSupplementaryContentObject {
    #[graphql(name = "type")]
    async fn content_type(&self) -> &String {
        &self.supplementary.content_type
    }

    async fn length(&self) -> Option<i64> {
        self.supplementary.content_length
    }

    async fn urls(&self) -> CollectionSupplementaryContentUrls {
        CollectionSupplementaryContentUrls {
            collection: self.collection.clone(),
            supplementary: self.supplementary.clone(),
        }
    }

    async fn text(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_collection_path(&self.collection, &self.supplementary.key).await?;
        Ok(ctx.storage.get(&path).await?)
    }

    async fn json(&self, ctx: &Context<'_>) -> Result<Value, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_collection_path(&self.collection, &self.supplementary.key).await?;
        let text = ctx.storage.get(&path).await?;
        Ok(serde_json::from_str(text.as_str())?)
    }
}

#[Object(name = "CollectionSupplementarySource")]
impl CollectionSupplementarySourceObject {
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

#[Object(name = "CollectionSupplementary")]
impl CollectionSupplementaryObject {
    async fn plan_id(&self) -> Option<String> { self.supplementary.plan_id.map(|id| id.to_string()) }

    async fn collection_id(&self) -> String {
        self.collection.id.to_string()
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

    async fn content(&self) -> CollectionSupplementaryContentObject {
        CollectionSupplementaryContentObject {
            collection: self.collection.clone(),
            supplementary: self.supplementary.clone(),
        }
    }

    async fn source(&self) -> CollectionSupplementarySourceObject {
        CollectionSupplementarySourceObject {
            supplementary: self.supplementary.clone(),
        }
    }
}
