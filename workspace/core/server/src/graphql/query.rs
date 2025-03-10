use crate::graphql::content::content::ContentObject;
use crate::graphql::queries::PersistedQueriesObject;
use crate::graphql::security::security::SecurityObject;
use crate::graphql::workflows::workflows::WorkflowsObject;
use async_graphql::*;
use log::error;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::configuration::configurations::ConfigurationsObject;
use crate::graphql::profiles::profiles::ProfilesObject;
use crate::graphql::server::ServerObject;
use crate::models::content::search::{SearchDocument, SearchQuery, SearchResultObject};
use crate::models::security::permission::PermissionAction;

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

    async fn search(
        &self,
        ctx: &Context<'_>,
        query: SearchQuery,
    ) -> Result<SearchResultObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let Ok(id) = Uuid::parse_str(query.storage_system_id.as_str()) else {
            return Ok(SearchResultObject {
                documents: vec![],
                estimated_hits: 0,
            });
        };
        let Some(storage_system) = ctx.workflow.get_storage_system(&id).await? else {
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
            search_query.with_filter(filter.as_str());
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
            let Some(id) = obj.get("_id") else {
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
                    content: obj
                        .get("_content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .trim()
                        .to_owned(),
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
                    content: obj
                        .get("_content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .trim()
                        .to_owned(),
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
                    content: obj
                        .get("_content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .trim()
                        .to_owned(),
                };
                documents.push(document);
            }
        }
        Ok(SearchResultObject {
            documents,
            estimated_hits: results
                .total_hits
                .unwrap_or(results.estimated_total_hits.unwrap_or(0))
                as i64,
        })
    }
}
