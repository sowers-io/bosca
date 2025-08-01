use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata_content_urls::MetadataContentUrls;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct MetadataContentObject {
    pub metadata: Metadata,
}

#[Object(name = "MetadataContent")]
impl MetadataContentObject {
    #[graphql(name = "type")]
    async fn content_type(&self) -> &String {
        &self.metadata.content_type
    }

    async fn length(&self, ctx: &Context<'_>) -> Result<Option<i64>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(self.metadata.content_length)
        } else {
            Ok(None)
        }
    }

    async fn urls(&self, ctx: &Context<'_>) -> Result<Option<MetadataContentUrls>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            Ok(Some(MetadataContentUrls {
                metadata: self.metadata.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn text(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
            Ok(Some(ctx.storage.get(&path).await?))
        } else {
            Ok(None)
        }
    }

    async fn json(&self, ctx: &Context<'_>) -> Result<Option<Value>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_metadata_content(
            self.metadata.clone(),
            PermissionAction::View,
        );
        if ctx.metadata_permission_check(check).await.is_ok() {
            let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
            let text = ctx.storage.get(&path).await?;
            Ok(Some(serde_json::from_str(text.as_str())?))
        } else {
            Ok(None)
        }
    }
}
