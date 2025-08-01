use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::categories::CategoriesObject;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_supplementary::CollectionSupplementaryObject;
use crate::graphql::content::collection_templates::CollectionTemplatesObject;
use crate::graphql::content::document_templates::DocumentTemplatesObject;
use crate::graphql::content::guide_templates::GuideTemplatesObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::metadata_supplementary::MetadataSupplementaryObject;
use crate::graphql::content::sources::SourcesObject;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::find_query::FindQueryInput;
use crate::models::content::slug::SlugType;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;

pub struct ContentObject {}

#[derive(Union)]
enum ContentItem {
    Metadata(MetadataObject),
    Collection(CollectionObject),
    Profile(ProfileObject),
}

#[Object(name = "Content")]
impl ContentObject {
    async fn slug(&self, ctx: &Context<'_>, slug: String) -> Result<Option<ContentItem>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(slug) = ctx.content.get_slug(&slug).await? {
            match slug.slug_type {
                SlugType::Metadata => {
                    let check =
                        PermissionCheck::new_with_metadata_id(slug.id, PermissionAction::View);
                    Ok(Some(ContentItem::Metadata(
                        ctx.metadata_permission_check(check).await?.into(),
                    )))
                }
                SlugType::Collection => {
                    let check =
                        PermissionCheck::new_with_collection_id(slug.id, PermissionAction::View);
                    Ok(Some(ContentItem::Collection(
                        ctx.collection_permission_check(check).await?.into(),
                    )))
                }
                SlugType::Profile => Ok(Some(ContentItem::Profile(
                    ctx.check_profile_action(&slug.id, PermissionAction::View)
                        .await?
                        .into(),
                ))),
            }
        } else {
            Ok(None)
        }
    }

    async fn find_collections(
        &self,
        ctx: &Context<'_>,
        query: FindQueryInput,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .find(&query)
            .await?
            .into_iter()
            .map(CollectionObject::new)
            .collect())
    }

    async fn find_collections_by_system(
        &self,
        ctx: &Context<'_>,
        query: FindQueryInput,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .find_system(&query)
            .await?
            .into_iter()
            .map(CollectionObject::new)
            .collect())
    }

    async fn find_collections_count(
        &self,
        ctx: &Context<'_>,
        mut query: FindQueryInput,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.collections.find_count(&mut query).await
    }

    async fn collection(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
    ) -> Result<Option<CollectionObject>, Error> {
        let id = match id {
            Some(id) => Uuid::parse_str(id.as_str()),
            None => Uuid::parse_str("00000000-0000-0000-0000-000000000000"),
        }?;
        let ctx = ctx.data::<BoscaContext>()?;
        let check = PermissionCheck::new_with_collection_id(id, PermissionAction::View);
        Ok(Some(ctx.collection_permission_check(check).await?.into()))
    }

    async fn find_metadata(
        &self,
        ctx: &Context<'_>,
        query: FindQueryInput,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .find(&query)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
    }

    async fn find_metadata_by_system(
        &self,
        ctx: &Context<'_>,
        query: FindQueryInput,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .find_system(&query)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
    }

    async fn find_metadata_count(
        &self,
        ctx: &Context<'_>,
        query: FindQueryInput,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.metadata.find_count(&query).await
    }

    async fn metadata(
        &self,
        ctx: &Context<'_>,
        id: String,
        version: Option<i32>,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::from_str(id.as_str())?;
        if let Some(version) = version {
            let check = PermissionCheck::new_with_metadata_id_with_version(
                id,
                version,
                PermissionAction::View,
            );
            Ok(Some(ctx.metadata_permission_check(check).await?.into()))
        } else {
            let check = PermissionCheck::new_with_metadata_id(id, PermissionAction::View);
            Ok(Some(ctx.metadata_permission_check(check).await?.into()))
        }
    }

    async fn metadata_supplementary(
        &self,
        ctx: &Context<'_>,
        supplementary_id: String,
    ) -> Result<Option<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::from_str(supplementary_id.as_str())?;
        let check = PermissionCheck::new_with_metadata_supplementary_id(
            supplementary_id,
            PermissionAction::View,
        );
        let (metadata, supplementary) = ctx.metadata_supplementary_permission_check(check).await?;
        Ok(Some(MetadataSupplementaryObject::new(
            metadata,
            supplementary,
        )))
    }

    async fn collection_supplementary(
        &self,
        ctx: &Context<'_>,
        supplementary_id: String,
    ) -> Result<Option<CollectionSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let supplementary_id = Uuid::from_str(supplementary_id.as_str())?;
        let check = PermissionCheck::new_with_collection_supplementary_id(
            supplementary_id,
            PermissionAction::View,
        );
        let (collection, supplementary) =
            ctx.collection_supplementary_permission_check(check).await?;
        Ok(Some(CollectionSupplementaryObject::new(
            collection,
            supplementary,
        )))
    }

    async fn document_templates(&self) -> DocumentTemplatesObject {
        DocumentTemplatesObject {}
    }

    async fn guide_templates(&self) -> GuideTemplatesObject {
        GuideTemplatesObject {}
    }

    async fn collection_templates(&self) -> CollectionTemplatesObject {
        CollectionTemplatesObject {}
    }

    async fn categories(&self) -> CategoriesObject {
        CategoriesObject {}
    }

    async fn sources(&self) -> SourcesObject {
        SourcesObject {}
    }

    async fn check_metadata_actions(
        &self,
        ctx: &Context<'_>,
        ids: Vec<String>,
        actions: Vec<PermissionAction>,
    ) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut passed = Vec::new();
        for id in ids {
            let id = Uuid::from_str(id.as_str())?;
            for action in &actions {
                let check = PermissionCheck::new_with_metadata_id(id, *action);
                if ctx.metadata_permission_check(check).await.is_ok() {
                    passed.push(id.to_string());
                }
            }
        }
        Ok(passed)
    }

    async fn check_collection_actions(
        &self,
        ctx: &Context<'_>,
        ids: Vec<String>,
        actions: Vec<PermissionAction>,
    ) -> Result<Vec<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut passed = Vec::new();
        for id in ids {
            let id = Uuid::from_str(id.as_str())?;
            for action in &actions {
                let check = PermissionCheck::new_with_collection_id(id, *action);
                if ctx.collection_permission_check(check).await.is_ok() {
                    passed.push(id.to_string());
                }
            }
        }
        Ok(passed)
    }
}
