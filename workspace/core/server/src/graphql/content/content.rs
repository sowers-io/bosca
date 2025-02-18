use crate::context::BoscaContext;
use crate::graphql::content::categories::CategoriesObject;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::sources::SourcesObject;
use crate::graphql::content::supplementary::MetadataSupplementaryObject;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::slug::SlugType;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;

pub struct ContentObject {}

#[derive(InputObject)]
pub struct FindAttributeInput {
    pub key: String,
    pub value: String,
}

#[derive(Union)]
enum ContentItem {
    Metadata(MetadataObject),
    Collection(CollectionObject),
    Profile(ProfileObject),
}

#[derive(Enum, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ExtensionFilterType {
    Document,
    DocumentTemplate,
    Guide,
    GuideTemplate,
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

    async fn find_collection(
        &self,
        ctx: &Context<'_>,
        attributes: Vec<FindAttributeInput>,
        category_ids: Option<Vec<String>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .collections
            .find(
                &attributes,
                category_ids.map(|c| c.iter().map(|c| Uuid::parse_str(c).unwrap()).collect()),
                limit,
                offset,
            )
            .await?
            .into_iter()
            .map(CollectionObject::new)
            .collect())
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
        attributes: Vec<FindAttributeInput>,
        content_types: Option<Vec<String>>,
        category_ids: Option<Vec<String>>,
        extension_filter: Option<ExtensionFilterType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .metadata
            .find(
                &attributes,
                &content_types,
                category_ids.map(|c| c.iter().map(|c| Uuid::parse_str(c).unwrap()).collect()),
                extension_filter,
                limit,
                offset,
            )
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
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
    ) -> Result<Option<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::from_str(id.as_str())?;
        let metadata = if let Some(version) = version {
            ctx
                .check_metadata_version_action(&id, version, PermissionAction::View)
                .await?
        } else {
            ctx
                .check_metadata_action(&id, PermissionAction::View)
                .await?
        };
        let supplementary = ctx
            .content
            .metadata
            .get_supplementary(&metadata.id, &key)
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

    async fn categories(&self) -> CategoriesObject {
        CategoriesObject {}
    }

    async fn sources(&self) -> SourcesObject {
        SourcesObject {}
    }
}
