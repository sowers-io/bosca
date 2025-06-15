use crate::context::BoscaContext;
use crate::models::content::search::{SearchDocument, SearchQuery, SearchResultFacet, SearchResultObject};
use crate::models::security::permission::PermissionAction;
use crate::search::query::{Hybrid, IndexQuery};
use async_graphql::Error;
use log::error;
use meilisearch_sdk::request::{HttpClient, Method};
use meilisearch_sdk::reqwest::ReqwestClient;
use meilisearch_sdk::search::SearchResults;
use serde_json::Value;
use std::default::Default;
use uuid::Uuid;

pub struct SearchClient {
    url: String,
    client: ReqwestClient,
}

impl SearchClient {
    pub fn new(
        url: String,
        api_key: String,
    ) -> Result<Self, Error> {
        Ok(SearchClient {
            url,
            client: ReqwestClient::new(Some(&api_key))?,
        })
    }

    pub async fn search(&self, ctx: &BoscaContext, query: &SearchQuery) -> Result<SearchResultObject, Error> {
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

        let offset = query.offset.unwrap_or(0);
        let mut limit = query.limit.unwrap_or(25);
        if limit > 100 {
            limit = 100;
        }

        let search_query = IndexQuery {
            q: query.query.clone(),
            limit,
            offset,
            filter: query.filter.clone(),
            sort: query.sort.clone(),
            facets: query.facets.clone(),
            hybrid: query.embedder.as_ref().map(|embedder| Hybrid { embedder: embedder.clone() }),
            ..Default::default()
        };

        let method = Method::Post {
            query: None::<String>,
            body: search_query,
        };

        let url = format!("{}/indexes/{}/search", self.url, index_name);
        let results: SearchResults<Value> = self.client.request(&url, method, 200).await?;

        let mut documents = Vec::new();
        for hit in results.hits {
            let obj = match hit.result {
                Value::Object(o) => Some(o),
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