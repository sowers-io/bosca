use crate::context::BoscaContext;
use crate::graphql::configuration::configurations::ConfigurationsObject;
use crate::graphql::content::content::ContentObject;
use crate::graphql::profiles::profiles::ProfilesObject;
use crate::graphql::queries::PersistedQueriesObject;
use crate::graphql::security::security::SecurityObject;
use crate::graphql::server::ServerObject;
use crate::graphql::workflows::workflows::WorkflowsObject;
use crate::models::content::search::{
    SearchDocument, SearchQuery, SearchResultFacet, SearchResultObject,
};
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use log::error;
use meilisearch_sdk::search::Selectors;
use uuid::Uuid;
use crate::graphql::cache::CacheObject;

pub struct QueryObject;

#[Object(name = "Query")]
impl QueryObject {
    async fn server(&self) -> ServerObject {
        ServerObject {}
    }

    async fn content(&self) -> ContentObject {
        ContentObject {}
    }

    async fn workflows(&self) -> WorkflowsObject {
        WorkflowsObject {}
    }

    async fn profiles(&self) -> ProfilesObject {
        ProfilesObject {}
    }

    async fn configurations(&self) -> ConfigurationsObject {
        ConfigurationsObject {}
    }

    async fn security(&self) -> SecurityObject {
        SecurityObject {}
    }

    async fn persisted_queries(&self) -> PersistedQueriesObject {
        PersistedQueriesObject {}
    }

    async fn cache(&self) -> CacheObject { CacheObject {} }

    async fn search(
        &self,
        ctx: &Context<'_>,
        query: SearchQuery,
    ) -> Result<SearchResultObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let storage_system = if let Some(storage_system_id) = query.storage_system_id.as_ref() {
            let Ok(id) = Uuid::parse_str(storage_system_id) else {
                return Ok(SearchResultObject {
                    documents: vec![],
                    facets: vec![],
                    estimated_hits: 0,
                });
            };
            let Some(storage_system) = ctx.workflow.get_storage_system(&id).await? else {
                return Err(Error::new("missing storage system"));
            };
            storage_system
        } else if let Some(storage_system_name) = query.storage_system_name.as_ref() {
            let Some(storage_system) = ctx
                .workflow
                .get_storage_system_by_name(storage_system_name)
                .await?
            else {
                return Err(Error::new("missing storage system"));
            };
            storage_system
        } else {
            return Err(Error::new("missing storage system"));
        };
        let Some(configuration) = storage_system.configuration else {
            return Err(Error::new("missing configuration"));
        };
        let index_name = configuration
            .get("indexName")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let index = ctx.search.index(index_name);
        let limit = query.limit.unwrap_or(25) as usize;
        let mut search_query = index.search();
        search_query.with_query(query.query.as_str());
        search_query.with_offset(query.offset.unwrap_or(0) as usize);
        search_query.with_limit(if limit > 100 { 100 } else { limit });
        if let Some(filter) = query.filter.as_ref() {
            if !filter.is_empty() {
                search_query.with_filter(filter.as_str());
            }
        }
        let sort_fields = query
            .sort
            .as_ref()
            .map(|s| s.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .unwrap_or_default();
        if !sort_fields.is_empty() {
            search_query.with_sort(sort_fields.as_slice());
        }
        let facet_fields = query
            .facets
            .as_ref()
            .map(|s| s.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .unwrap_or_default();
        if !facet_fields.is_empty() {
            search_query.with_facets(Selectors::Some(facet_fields.as_slice()));
        }
        let results = search_query.execute::<serde_json::Value>().await?;
        let mut documents = Vec::new();
        for hit in results.hits {
            let obj = match hit.result {
                serde_json::Value::Object(o) => Some(o),
                _ => None,
            };
            if obj.is_none() {
                continue;
            }
            let obj = obj.unwrap();
            let Some(id) = obj.get("id") else {
                return Err(Error::new("missing id"));
            };
            let id = id.as_str().unwrap();
            let Ok(id) = Uuid::parse_str(id) else {
                error!("failed to parse id: {}", id);
                continue;
            };
            let hit_type = obj.get("_type").unwrap().as_str().unwrap();
            if hit_type == "metadata" {
                let metadata = ctx.check_metadata_action(&id, PermissionAction::View).await;
                if metadata.is_err() {
                    continue;
                }
                let document = SearchDocument {
                    metadata: Some(metadata?),
                    collection: None,
                    profile: None,
                };
                documents.push(document);
            } else if hit_type == "collection" {
                let collection = ctx
                    .check_collection_action(&id, PermissionAction::View)
                    .await;
                if collection.is_err() {
                    continue;
                }
                let document = SearchDocument {
                    metadata: None,
                    collection: Some(collection?),
                    profile: None,
                };
                documents.push(document);
            } else if hit_type == "profile" {
                let profile = ctx.check_profile_action(&id, PermissionAction::View).await;
                if profile.is_err() {
                    continue;
                }
                let document = SearchDocument {
                    metadata: None,
                    collection: None,
                    profile: Some(profile?),
                };
                documents.push(document);
            }
        }
        let mut facets = Vec::new();
        if let Some(distribution) = results.facet_distribution {
            for (field, value) in distribution {
                for (value, count) in value {
                    facets.push(SearchResultFacet {
                        field: field.to_string(),
                        value: value.to_string(),
                        count: count as i64,
                    })
                }
            }
        }
        Ok(SearchResultObject {
            documents,
            facets,
            estimated_hits: results
                .total_hits
                .unwrap_or(results.estimated_total_hits.unwrap_or(0))
                as i64,
        })
    }
}
