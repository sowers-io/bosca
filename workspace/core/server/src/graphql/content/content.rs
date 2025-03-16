use crate::context::BoscaContext;
use crate::graphql::content::categories::CategoriesObject;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::collection_templates::CollectionTemplatesObject;
use crate::graphql::content::document_templates::DocumentTemplatesObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::sources::SourcesObject;
use crate::graphql::content::metadata_supplementary::MetadataSupplementaryObject;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::slug::SlugType;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;
use crate::graphql::content::guide_templates::GuideTemplatesObject;
use crate::models::content::find_query::FindQueryInput;

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
                SlugType::Metadata => Ok(Some(ContentItem::Metadata(
                    ctx.check_metadata_action(&slug.id, PermissionAction::View)
                        .await?
                        .into(),
                ))),
                SlugType::Collection => Ok(Some(ContentItem::Collection(
                    ctx.check_collection_action(&slug.id, PermissionAction::View)
                        .await?
                        .into(),
                ))),
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
        mut query: FindQueryInput,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .find(&mut query)
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
        Ok(Some(
            ctx.check_collection_action(&id, PermissionAction::View)
                .await?
                .into(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    async fn find_metadata(
        &self,
        ctx: &Context<'_>,
        mut query: FindQueryInput,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .find(&mut query)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
    }

    async fn find_metadata_count(
        &self,
        ctx: &Context<'_>,
        mut query: FindQueryInput,
    ) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.content.metadata.find_count(&mut query).await
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
            Ok(Some(
                ctx.check_metadata_version_action(&id, version, PermissionAction::View)
                    .await?
                    .into(),
            ))
        } else {
            Ok(Some(
                ctx.check_metadata_action(&id, PermissionAction::View)
                    .await?
                    .into(),
            ))
        }
    }

    async fn metadata_supplementary(
        &self,
        ctx: &Context<'_>,
        id: String,
        version: Option<i32>,
        key: String,
        plan_id: Option<String>,
    ) -> Result<Option<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::from_str(id.as_str())?;
        let plan_id = plan_id.map(|p| Uuid::from_str(p.as_str()).unwrap());
        let metadata = if let Some(version) = version {
            ctx.check_metadata_version_action(&id, version, PermissionAction::View)
                .await?
        } else {
            ctx.check_metadata_action(&id, PermissionAction::View)
                .await?
        };
        let supplementary = ctx
            .content
            .metadata_supplementary
            .get_supplementary(&metadata.id, &key, plan_id)
            .await?;
        if let Some(supplementary) = supplementary {
            Ok(Some(MetadataSupplementaryObject::new(
                metadata,
                supplementary,
            )))
        } else {
            Ok(None)
        }
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
}
