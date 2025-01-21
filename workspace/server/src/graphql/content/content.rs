use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::content::source::SourceObject;
use crate::graphql::content::supplementary::MetadataSupplementaryObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use std::str::FromStr;
use uuid::Uuid;
use serde_json::Value;
use crate::context::BoscaContext;
use crate::models::content::search::{SearchDocument, SearchQuery, SearchResultObject};

pub struct ContentObject {}

#[derive(InputObject)]
pub struct FindAttributeInput {
    pub key: String,
    pub value: String,
}

#[Object(name = "Content")]
impl ContentObject {
    async fn find_collection(
        &self,
        ctx: &Context<'_>,
        attributes: Vec<FindAttributeInput>,
    ) -> Result<Vec<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .find_collections(&attributes)
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

    async fn find_metadata(
        &self,
        ctx: &Context<'_>,
        attributes: Vec<FindAttributeInput>,
    ) -> Result<Vec<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .find_metadata(&attributes)
            .await?
            .into_iter()
            .map(MetadataObject::new)
            .collect())
    }

    async fn metadata(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::from_str(id.as_str())?;
        Ok(Some(
            ctx.check_metadata_action(&id, PermissionAction::View)
                .await?
                .into(),
        ))
    }

    async fn metadata_supplementary(
        &self,
        ctx: &Context<'_>,
        id: String,
        key: String,
    ) -> Result<Option<MetadataSupplementaryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::from_str(id.as_str())?;
        let metadata = ctx.check_metadata_action(&id, PermissionAction::View).await?;
        let supplementary = ctx.content
            .get_metadata_supplementary(&metadata.id, &key)
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

    async fn sources(&self, ctx: &Context<'_>) -> Result<Vec<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.content
            .get_sources()
            .await?
            .into_iter()
            .map(SourceObject::new)
            .collect())
    }

    async fn source(&self, ctx: &Context<'_>, id: String) -> Result<Option<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(match Uuid::parse_str(id.as_str()) {
            Ok(id) => ctx.content.get_source_by_id(&id).await?,
            Err(_) => ctx.content.get_source_by_name(&id).await?,
        }
        .map(|s| s.into()))
    }

    async fn search(&self, ctx: &Context<'_>, query: SearchQuery) -> Result<SearchResultObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(query.storage_system_id.as_str())?;
        let Some(storage_system) = ctx.workflow.get_storage_system(&id).await? else {
            return Err(Error::new("missing storage system"));
        };
        let Some(configuration) = storage_system.configuration else {
            return Err(Error::new("missing configuration"));
        };
        let index_name = configuration.get("indexName").unwrap().as_str().unwrap().to_string();
        let index = ctx.search.index(index_name);
        let limit = query.limit.unwrap_or(25) as usize;
        let mut search_query = index.search();
        search_query.with_query(query.query.as_str());
        search_query.with_offset(query.offset.unwrap_or(0) as usize);
        search_query.with_limit(if limit > 100 { 100 } else { limit });
        if let Some(filter) = query.filter.as_ref() {
            search_query.with_filter(filter.as_str());
        }
        let results = search_query.execute::<Value>().await?;
        let mut documents = Vec::new();
        for hit in results.hits {
            let obj = match hit.result {
                Value::Object(o) => Some(o),
                _ => None
            };
            if obj.is_none() {
                continue
            }
            let obj = obj.unwrap();
            let id = Uuid::parse_str(obj.get("_id").unwrap().as_str().unwrap())?;
            let hit_type = obj.get("_type").unwrap().as_str().unwrap();
            if hit_type == "metadata" {
                let metadata = ctx.check_metadata_action(&id, PermissionAction::View).await;
                if metadata.is_err() {
                    continue;
                }
                let document = SearchDocument {
                    metadata: Some(metadata?),
                    collection: None,
                    content: obj.get("_content").unwrap().as_str().unwrap().trim().to_owned(),
                };
                documents.push(document);
            } else if hit_type == "collection" {
                let collection = ctx.check_collection_action(&id, PermissionAction::View).await;
                if collection.is_err() {
                    continue;
                }
                let document = SearchDocument {
                    metadata: None,
                    collection: Some(collection?),
                    content: obj.get("_content").unwrap().as_str().unwrap().trim().to_owned(),
                };
                documents.push(document);
            }
        }
        Ok(SearchResultObject {
            documents,
            estimated_hits: results.total_hits.unwrap_or(results.estimated_total_hits.unwrap_or(0)) as i64
        })
    }
}
